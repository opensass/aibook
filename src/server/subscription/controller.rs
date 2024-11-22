#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::db::get_client;
#[cfg(feature = "server")]
use crate::pay::get_stripe;
use crate::server::auth::model::User;
use crate::server::book::model::Book;
use crate::server::common::response::SuccessResponse;
use crate::server::subscription::model::Subscription;
use crate::server::subscription::request::StripeCancelRequest;
use crate::server::subscription::request::StripePaymentRequest;
use crate::server::subscription::request::SubscriptionDetailRequest;
use crate::server::subscription::request::UpdateSubscriptionRequest;
use crate::server::subscription::response::SubscriptionDetailResponse;
use chrono::prelude::*;
use std::str::FromStr;
#[cfg(feature = "server")]
use stripe::{
    CheckoutSessionMode, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    Subscription as StripeSubscription, SubscriptionId,
};

#[server]
pub async fn get_subscription_detail(
    req: SubscriptionDetailRequest,
) -> Result<SuccessResponse<SubscriptionDetailResponse>, ServerFnError> {
    let client = get_client().await;
    let stripe_client = get_stripe().await.lock().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let sub_collection = db.collection::<Subscription>("subscriptions");

    if let Some(subscription) = sub_collection
        .find_one(doc! { "user": req.user_id })
        .await?
    {
        // TODO: Change to enum val. Right now only stripe is allowed, might add paypal for the pal in the future
        if subscription.method == "stripe" {
            let subscription_id = SubscriptionId::from_str(subscription.sub_id.as_str())
                .map_err(|_| ServerFnError::new("Invalid subscription ID"))?;
            let stripe_sub =
                StripeSubscription::retrieve(&stripe_client, &subscription_id, &[]).await?;

            return Ok(SuccessResponse {
                status: "success".into(),
                data: SubscriptionDetailResponse {
                    session: serde_json::to_string(&stripe_sub)?,
                    method: subscription.method,
                },
            });
        }
    }

    Err(ServerFnError::new(
        "Subscription not found or not a Stripe subscription",
    ))
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
pub async fn start_stripe_payment(
    req: StripePaymentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let stripe_client = get_stripe().await.lock().await;

    let mut session = CreateCheckoutSession::new();
    let success_url = format!(
        "{}/success",
        std::env::var("WEBSITE_URL").expect("WEBSITE_URL must be set.")
    );
    session.success_url = Some(&success_url);
    let cancel_url = format!(
        "{}/failed",
        std::env::var("WEBSITE_URL").expect("WEBSITE_URL must be set.")
    );
    session.cancel_url = Some(&cancel_url);
    session.mode = Some(CheckoutSessionMode::Subscription);
    session.line_items = vec![CreateCheckoutSessionLineItems {
        price: Some(req.plan_id.clone()),
        quantity: Some(1),
        ..Default::default()
    }]
    .into();

    let checkout_session = stripe::CheckoutSession::create(&stripe_client, session).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: checkout_session.url.unwrap_or_default(),
    })
}

#[server]
pub async fn stripe_cancel(
    req: StripeCancelRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let stripe_client = get_stripe().await.lock().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let sub_collection = db.collection::<Subscription>("subscriptions");
    let user_collection = db.collection::<User>("users");

    if let Some(subscription) = sub_collection.find_one(doc! { "subId": &req.id }).await? {
        let subscription_id = SubscriptionId::from_str(req.id.as_str())
            .map_err(|_| ServerFnError::new("Invalid subscription ID"))?;
        let user_id = subscription.user;

        StripeSubscription::delete(&stripe_client, &subscription_id).await?;

        user_collection
            .update_one(doc! { "_id": user_id }, doc! { "$set": { "type": "free" } })
            .await?;

        sub_collection.delete_one(doc! { "subId": &req.id }).await?;

        return Ok(SuccessResponse {
            status: "success".into(),
            data: "Subscription canceled successfully".into(),
        });
    }

    Err(ServerFnError::new("Subscription not found"))
}
