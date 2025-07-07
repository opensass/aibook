use anyhow::Result;
use dioxus::prelude::*;
use http_api_isahc_client::IsahcClient;
use redis::{AsyncCommands, Script};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, OnceCell};

pub struct RateLimiter {
    client: redis::Client,
    max: u32,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(redis_url: &str, max: u32, window_secs: u64) -> Result<Self> {
        Ok(Self {
            client: redis::Client::open(redis_url)?,
            max,
            window_secs,
        })
    }

    pub async fn check(&self, ip: &str) -> Result<bool> {
        let key = format!("rl:{}", ip);

        // lua script for rate limiting
        static SCRIPT: &str = r#"
            local key         = KEYS[1]
            local requests    = tonumber(redis.call('GET', key) or '-1')
            local max_requests = tonumber(ARGV[1])
            local expiry       = tonumber(ARGV[2])

            if requests == -1 then
                redis.call('INCR', key)
                redis.call('EXPIRE', key, expiry)
                return true
            elseif requests < max_requests then
                redis.call('INCR', key)
                return true
            else
                return false
            end
        "#;

        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let current: i64 = Script::new(SCRIPT)
            .key(&key)
            .arg(self.max as i64)
            .arg(self.window_secs as i64)
            .invoke_async(&mut conn)
            .await?;

        Ok(current as u32 <= self.max)
    }
}

static REDIS: OnceCell<Mutex<RateLimiter>> = OnceCell::const_new();

async fn init_redis() -> &'static Mutex<RateLimiter> {
    REDIS
        .get_or_init(|| async {
            let client = RateLimiter::new(
                &std::env::var("REDIS_URL").expect("REDIS_URL must be set."),
                30,
                60,
            )
            .unwrap();
            Mutex::new(client)
        })
        .await
}

pub async fn get_redis_client() -> &'static Mutex<RateLimiter> {
    init_redis().await
}

#[derive(Deserialize)]
struct Ipify {
    ip: String,
}

pub async fn fetch_public_ip() -> Result<String, ServerFnError> {
    let Ipify { ip } = reqwest::get("https://api.ipify.org?format=json")
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .json::<Ipify>()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(ip)
}
