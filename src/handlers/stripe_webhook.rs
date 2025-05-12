use crate::{errors::AppResult, state::AppState};
use anyhow::Context;
use axum::body::Bytes;
use axum::http::StatusCode;
use axum::{Extension, http::HeaderMap, response::IntoResponse};
use marzban_api::models::proxy::ProxyTypes;
use marzban_api::models::user::{
    Inbounds, Proxies, Shadowsocks, Trojan, UserCreate, UserModify, UserStatusCreate,
    UserStatusModify, Vless, Vmess,
};
use stripe::{Event, EventObject, EventType, Webhook};
use tracing::{debug, error};

/// POST `/api/stripe/` - Stripe webhook endpoint
pub async fn stripe_webhook(
    Extension(app_state): Extension<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<impl IntoResponse> {
    // Verify Stripe signature
    let signature = match headers.get("stripe-signature") {
        Some(sig) => sig.to_str().unwrap_or("").to_string(),
        None => {
            error!("Missing stripe-signature header");
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
    };

    let payload_str = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(e) => {
            error!("Invalid UTF-8 in webhook body: {e}");
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
    };

    let event: Event = match Webhook::construct_event(
        payload_str,
        &signature,
        std::env::var("STRIPE_WEBHOOK_SECRET")
            .unwrap_or_default()
            .as_str(),
    ) {
        Ok(event) => event,
        Err(e) => {
            error!("Failed to verify Stripe webhook: {e}");
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
    };

    debug!("Received Stripe event: {:?}", event);

    // Only handle PaymentIntent events
    match event.type_ {
        EventType::PaymentIntentSucceeded
        | EventType::PaymentIntentPaymentFailed
        | EventType::PaymentIntentCanceled
        | EventType::PaymentIntentRequiresAction
        | EventType::PaymentIntentRequiresCapture
        | EventType::PaymentIntentProcessing => {
            if let EventObject::PaymentIntent(payment_intent) = event.data.object {
                let payment_intent_id = payment_intent.id.to_string();
                use crate::payloads::enums::UserTransactionStatus;
                let new_status = match event.type_ {
                    EventType::PaymentIntentSucceeded => UserTransactionStatus::Succeeded,
                    EventType::PaymentIntentPaymentFailed => {
                        UserTransactionStatus::RequiresPaymentMethod
                    }
                    EventType::PaymentIntentCanceled => UserTransactionStatus::Canceled,
                    EventType::PaymentIntentRequiresAction => UserTransactionStatus::RequiresAction,
                    EventType::PaymentIntentRequiresCapture => {
                        UserTransactionStatus::RequiresCapture
                    }
                    EventType::PaymentIntentProcessing => UserTransactionStatus::Processing,
                    _ => return Ok(StatusCode::OK.into_response()),
                };
                // Update transaction status in DB
                if let Err(e) = app_state
                    .transaction_repository()
                    .change_transaction_status_by_payment_intent(&payment_intent_id, new_status)
                    .await
                {
                    error!("Failed to update transaction status: {e}");
                    return Err(anyhow::anyhow!(e).into());
                }

                // Update user's Marzban subscription status
                if event.type_ == EventType::PaymentIntentSucceeded {
                    // Get the user ID from the metadata
                    let user_id = payment_intent
                        .metadata
                        .get("user_id")
                        .context("Failed to get user_id from metadata")?
                        .parse::<u64>()
                        .context("Failed to parse user_id from metadata")?;

                    // Get the user from the database
                    let user = app_state
                        .user_repository()
                        .get_user_by_id(user_id as i64)
                        .await
                        .context("Failed to get user")?
                        .context("User no longer exists")?;

                    // Get the plan ID from metadata
                    let plan_id = payment_intent
                        .metadata
                        .get("plan_id")
                        .context("Failed to get plan_id from metadata")?
                        .parse::<u64>()
                        .context("Failed to parse plan_id from metadata")?;

                    let plan = app_state
                        .plan_repository()
                        .get_plan_by_id(plan_id as i64)
                        .await
                        .context("Failed to get plan")?
                        .context("Plan no longer exists")?;

                    // Check if the user has a Marzban account
                    let marzban_username = app_state
                        .user_repository()
                        .get_marzban_username(user_id as i64)
                        .await
                        .context("Failed to get Marzban username")?;

                    let (inbounds, proxies) = inbounds_and_proxies(app_state.clone()).await;

                    // Now actually (create the user if required, and) update the user's plan
                    // also holy sphaghetti code batman
                    match marzban_username {
                        None => {
                            app_state.marzban_client()
                                .add_user(&UserCreate {
                                    proxies,
                                    expire: {
                                        match plan.duration_days {
                                            0 => None,
                                            _ => Some(
                                                chrono::Utc::now()
                                                    .checked_add_signed(chrono::Duration::days(
                                                        plan.duration_days as i64,
                                                    ))
                                                    .expect("Failed to add days")
                                                    .timestamp()
                                                    as u64,
                                            ),
                                        }
                                    },
                                    data_limit: {
                                        // Convert data limit to bytes (where kb = 1024 bytes)
                                        plan.data_limit as u64 * 1024 * 1024 * 1024
                                    },
                                    data_limit_reset_strategy:
                                        marzban_api::models::user::UserDataLimitResetStrategy::NoReset,
                                    inbounds,
                                    note: Some(format!("Created by HiddN. ID: {}", user_id)),
                                    sub_updated_at: None,
                                    sub_last_user_agent: None,
                                    online_at: None,
                                    on_hold_expire_duration: None,
                                    on_hold_timeout: None,
                                    auto_delete_in_days: None,
                                    username: user.email.clone(),
                                    status: UserStatusCreate::Active,
                                })
                                .await
                                .expect("Failed to create user");

                            // Also add the marzban username to the user in db
                            app_state
                                .user_repository()
                                .update_marzban_username(user_id as i64, &user.email.clone())
                                .await
                                .context("Failed to update Marzban username")?;
                        }
                        Some(marzban_username) => {
                            // Get the user's current plan
                            let current_plan = app_state
                                .marzban_client()
                                .get_user(&marzban_username)
                                .await
                                .expect("Failed to get user");

                            // If user's expiration is 'never', keep it at 'never'.
                            // If user's expiration is in the past OR data limit has been reached, set it to the new plan's expiration
                            // If user's expiration is in the future, add the new plan's duration to it

                            // If user's data limit is 0, keep it as 0
                            // If user's data limit is not 0 and limit has not been reached, add the new plan's data limit to it
                            // If user's data limit is not 0 and limit has been reached, set it to the new plan's data limit

                            let mut reset_data_usage = false;

                            let new_data_limit = match current_plan.data_limit {
                                None => 0,
                                Some(data_limit) if current_plan.used_traffic >= data_limit => {
                                    // If data limit has been reached, set it to the new plan's data limit
                                    reset_data_usage = true;
                                    plan.data_limit as u64 * 1024 * 1024 * 1024
                                }
                                Some(data_limit) => {
                                    // If data limit has not been reached, add the new plan's data limit to it
                                    data_limit + plan.data_limit as u64 * 1024 * 1024 * 1024
                                }
                            };

                            let new_expiration = match current_plan.expire {
                                // If expiration is 'never', keep it as 'never'
                                None => 0,
                                Some(expire) => {
                                    if expire < chrono::Utc::now().timestamp() as u64
                                        || reset_data_usage
                                    {
                                        // If expiration is in the past, set it to the new plan's expiration
                                        // Or, if data limit has been reached, set it to the new plan's expiration
                                        chrono::Utc::now()
                                            .checked_add_signed(chrono::Duration::days(
                                                plan.duration_days as i64,
                                            ))
                                            .expect("Failed to add days")
                                            .timestamp()
                                            as u64
                                    } else {
                                        // If expiration is in the future, add the new plan's duration to it
                                        chrono::DateTime::from_timestamp(expire as i64, 0)
                                            .expect("Failed to convert to chrono")
                                            .checked_add_signed(chrono::Duration::days(
                                                plan.duration_days as i64,
                                            ))
                                            .expect("Failed to add days")
                                            .timestamp()
                                            as u64
                                    }
                                }
                            };

                            // Update the user's plan
                            app_state.marzban_client().modify_user(&marzban_username, &UserModify {
                                                proxies,
                                                expire: Some(new_expiration),
                                                data_limit: new_data_limit,
                                                data_limit_reset_strategy:
                                                    marzban_api::models::user::UserDataLimitResetStrategy::NoReset,
                                                inbounds,
                                                note: Some(format!("Updated by HiddN. ID: {}", user_id)),
                                                sub_updated_at: None,
                                                sub_last_user_agent: None,
                                                online_at: None,
                                                on_hold_expire_duration: None,
                                                on_hold_timeout: None,
                                                auto_delete_in_days: None,
                                                status: UserStatusModify::Active,
                                            }).await.expect("Failed to update user");

                            // If reset_data_usage is true, reset the user's data usage
                            if reset_data_usage {
                                app_state
                                    .marzban_client()
                                    .reset_user_data_usage(&marzban_username)
                                    .await
                                    .expect("Failed to reset user data usage");
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }

    Ok(StatusCode::OK.into_response())
}

/// Helper function to output Proxies
async fn inbounds_and_proxies(app_state: AppState) -> (Inbounds, Proxies) {
    // Get list of inbounds available
    let inbounds = app_state
        .marzban_client()
        .get_inbounds()
        .await
        .expect("Failed to get inbounds");

    let inbounds = Inbounds {
        trojan: if inbounds
            .get(&ProxyTypes::Trojan)
            .is_none_or(|x| x.is_empty())
        {
            None
        } else {
            Some(
                inbounds[&ProxyTypes::Trojan]
                    .iter()
                    .map(|x| x.tag.clone())
                    .collect(),
            )
        },
        vless: if inbounds
            .get(&ProxyTypes::Vless)
            .is_none_or(|x| x.is_empty())
        {
            None
        } else {
            Some(
                inbounds[&ProxyTypes::Vless]
                    .iter()
                    .map(|x| x.tag.clone())
                    .collect(),
            )
        },
        vmess: if inbounds
            .get(&ProxyTypes::Vmess)
            .is_none_or(|x| x.is_empty())
        {
            None
        } else {
            Some(
                inbounds[&ProxyTypes::Vmess]
                    .iter()
                    .map(|x| x.tag.clone())
                    .collect(),
            )
        },
        shadowsocks: if inbounds
            .get(&ProxyTypes::ShadowSocks)
            .is_none_or(|x| x.is_empty())
        {
            None
        } else {
            Some(
                inbounds[&ProxyTypes::ShadowSocks]
                    .iter()
                    .map(|x| x.tag.clone())
                    .collect(),
            )
        },
    };

    let proxies = Proxies {
        trojan: if inbounds.trojan.is_none() {
            None
        } else {
            Some(Trojan {
                password: None,
                flow: None,
            })
        },
        vless: if inbounds.vless.is_none() {
            None
        } else {
            Some(Vless {
                id: None,
                flow: Some("xtls-rprx-vision".to_string()),
            })
        },
        vmess: if inbounds.vmess.is_none() {
            None
        } else {
            Some(Vmess {
                id: None,
                security: None,
            })
        },
        shadowsocks: if inbounds.shadowsocks.is_none() {
            None
        } else {
            Some(Shadowsocks {
                password: None,
                method: None,
            })
        },
    };

    (inbounds, proxies)
}
