#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::db::get_client;
use crate::server::auth::model::User;
use crate::server::book::model::Book;
use crate::server::common::response::SuccessResponse;
use crate::server::subscription::model::Subscription;
use crate::server::subscription::request::StripePaymentRequest;
use crate::server::subscription::request::SubscriptionDetailRequest;
use crate::server::subscription::request::UpdateSubscriptionRequest;
use crate::server::subscription::response::SubscriptionDetailResponse;
use chrono::prelude::*;

#[server]
pub async fn get_subscription_detail(
    req: SubscriptionDetailRequest,
) -> Result<SuccessResponse<SubscriptionDetailResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let sub_collection = db.collection::<Subscription>("subscriptions");

    let subscription = sub_collection
        .find_one(doc! { "user": req.user_id })
        .await?;

    if let Some(sub) = subscription {
        if sub.method == "stripe" {
            let stripe_sub = fetch_stripe_subscription(&sub.sub_id).await?;
            return Ok(SuccessResponse {
                status: "success".into(),
                data: SubscriptionDetailResponse {
                    session: stripe_sub,
                    method: sub.method,
                },
            });
        }
    }

    Err(ServerFnError::new(
        "Subscription not found or not a Stripe subscription",
    ))
}

async fn fetch_stripe_subscription(subscriber_id: &str) -> Result<String, ServerFnError> {
    Ok("stripe_session".into())
}

#[server]
pub async fn update_subscription(
    req: UpdateSubscriptionRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let sub_collection = db.collection::<Subscription>("subscriptions");
    let user_collection = db.collection::<User>("users");

    let subscription = sub_collection
        .find_one(doc! { "sub": &req.subscription_id })
        .await?;

    if let Some(sub) = subscription {
        user_collection
            .update_one(
                doc! { "_id": sub.user },
                doc! { "$set": { "role": "free" } },
            )
            .await?;

        sub_collection
            .delete_one(doc! { "sub": &req.subscription_id })
            .await?;
    }

    Ok(SuccessResponse {
        status: "success".into(),
        data: "Subscription updated and deleted".into(),
    })
}

#[server]
pub async fn stripe_payment(
    req: StripePaymentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let session_url = create_stripe_session(&req.plan_id).await?;
    Ok(SuccessResponse {
        status: "success".into(),
        data: session_url,
    })
}

async fn create_stripe_session(plan_id: &str) -> Result<String, ServerFnError> {
    Ok("stripe_session_url".into())
}
