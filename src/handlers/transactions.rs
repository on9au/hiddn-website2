use crate::payloads::{CreateOrder, CreateOrderResponse, UserTransactionStatus};
use crate::sessions::AuthSession;
use crate::{errors::AppResult, state::AppState};
use anyhow::Context;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json, response::IntoResponse};
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use stripe::{CancelPaymentIntent, CreatePaymentIntent, PaymentIntent, PaymentIntentId};

/// GET `/api/transactions`
pub async fn get_transactions(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    let txs = app_state
        .transaction_repository()
        .get_transactions_by_user_id(user.id)
        .await?;
    Ok(Json(txs))
}

/// GET `/api/transactions/:id`
pub async fn get_transaction_by_id(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    match app_state
        .transaction_repository()
        .get_transaction_by_id(id as i64)
        .await?
    {
        Some(tx) => {
            // Check if transaction belongs to the user
            if tx.user_id as i64 != user.id {
                return Ok((StatusCode::FORBIDDEN, "Forbidden").into_response());
            }

            Ok(Json(tx).into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "Transaction not found").into_response()),
    }
}

/// GET `/api/transactions/:id/secret`
pub async fn get_transaction_secret(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    match app_state
        .transaction_repository()
        .get_transaction_by_id(id as i64)
        .await?
    {
        Some(tx) => {
            // Check if transaction belongs to the user
            if tx.user_id as i64 != user.id {
                return Ok((StatusCode::FORBIDDEN, "Forbidden").into_response());
            }

            // Check if transaction has a payment intent
            let payment_intent = match tx.stripe_payment_intent_id {
                Some(ref intent) => intent,
                None => return Ok((StatusCode::NOT_FOUND, "No payment intent").into_response()),
            };

            // Get the payment intent
            let payment_intent_id = PaymentIntentId::from_str(payment_intent)
                .context("Failed to parse payment intent id")?;
            let payment_intent =
                PaymentIntent::retrieve(app_state.stripe_client(), &payment_intent_id, &[])
                    .await
                    .context("Failed to retrieve payment intent")?;

            Ok(Json(payment_intent.client_secret).into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "Transaction not found").into_response()),
    }
}

/// POST `/api/transactions/:id/complete`
pub async fn complete_transaction(Path(_id): Path<u32>) -> AppResult<impl IntoResponse> {
    // Respectfully, what the fuck is the point of this handler?
    Ok(StatusCode::OK.into_response())
}

/// POST `/api/transactions/:id/cancel`
pub async fn cancel_transaction(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
    Path(id): Path<u32>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    match app_state
        .transaction_repository()
        .get_transaction_by_id(id as i64)
        .await?
    {
        Some(tx) => {
            // Check if transaction belongs to the user
            if tx.user_id as i64 != user.id {
                return Ok((StatusCode::FORBIDDEN, "Forbidden").into_response());
            }

            // Check if transaction has a payment intent
            let payment_intent = match tx.stripe_payment_intent_id {
                Some(ref intent) => intent,
                None => return Ok((StatusCode::NOT_FOUND, "No payment intent").into_response()),
            };

            // Cancel the payment intent
            let payment_intent_id = stripe::PaymentIntentId::from_str(payment_intent)
                .context("Failed to parse payment intent id")?;

            PaymentIntent::cancel(
                app_state.stripe_client(),
                &payment_intent_id,
                CancelPaymentIntent {
                    cancellation_reason: Some(
                        stripe::PaymentIntentCancellationReason::RequestedByCustomer,
                    ),
                },
            )
            .await
            .context("Failed to cancel payment intent")?;

            // Update the transaction in the database
            app_state
                .transaction_repository()
                .change_transaction_status(id as i64, UserTransactionStatus::Cancelled)
                .await?;

            Ok(StatusCode::OK.into_response())
        }
        None => Ok((StatusCode::NOT_FOUND, "Transaction not found").into_response()),
    }
}

/// POST `/api/transactions/`
pub async fn create_transaction(
    auth_session: AuthSession,
    Extension(app_state): Extension<AppState>,
    Json(payload): Json<CreateOrder>,
) -> AppResult<impl IntoResponse> {
    let user = auth_session.user.context("Failed to get user")?;

    // Get the plan
    let plan = match app_state
        .plan_repository()
        .get_plan_by_id(payload.plan_id as i64)
        .await?
    {
        Some(plan) => plan,
        None => return Ok((StatusCode::NOT_FOUND, "Plan not found").into_response()),
    };

    let plan_price: i64 = (plan.price * 100_i32)
        .to_i64()
        .context("Failed to convert plan price")?;

    // Create a payment intent
    let mut payment_intent = CreatePaymentIntent::new(plan_price, stripe::Currency::AUD);
    payment_intent.statement_descriptor_suffix = Some("Payment for HiddN Plan");
    payment_intent.receipt_email = Some(user.email.as_str());
    payment_intent.metadata = Some(
        [
            ("plan_id".to_string(), plan.id.to_string()),
            ("plan_name".to_string(), plan.name.clone()),
            ("user_id".to_string(), user.id.to_string()),
            ("email".to_string(), user.email.clone()),
        ]
        .iter()
        .cloned()
        .collect(),
    );

    let payment_intent = PaymentIntent::create(app_state.stripe_client(), payment_intent)
        .await
        .context("Failed to create payment intent")?;

    let payment_intent_status: UserTransactionStatus = payment_intent.status.into();

    // Add the transaction to the database
    let transaction_id = app_state
        .transaction_repository()
        .create_transaction(
            user.id,
            plan.id as i64,
            payment_intent.id.to_string(),
            plan_price,
            payment_intent_status,
        )
        .await?;

    Ok(Json(CreateOrderResponse {
        order_id: transaction_id,
        payment_intent_client_secret: payment_intent
            .client_secret
            .context("Failed to get client secret")?, // Should not unwrap usually
    })
    .into_response())
}
