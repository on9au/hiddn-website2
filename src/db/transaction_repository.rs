use crate::payloads::enums::UserTransactionStatus;
use crate::payloads::transaction_payloads::UserTransaction;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::MySqlPool;
use sqlx::types::BigDecimal;

pub struct TransactionRepository {
    pool: MySqlPool,
}

impl TransactionRepository {
    /// Creates a new `TransactionRepository` instance.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// # WARNING
    ///
    /// This will return **ALL** transactions in the database.
    ///
    /// Want to only return transactions for a specific user?
    ///
    /// Use [Self::get_transactions_by_user_id] instead.
    pub async fn get_transactions(&self) -> Result<Vec<UserTransaction>> {
        let transactions = sqlx::query_as!(
            UserTransaction,
            r#"
            SELECT 
                id as `id: u32`,
                user_id as `user_id: u32`,
                plan_id as `plan_id: u32`,
                stripe_payment_intent_id as `stripe_payment_intent_id: String`,
                amount as `amount: BigDecimal`,
                status as `status: UserTransactionStatus`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`
            FROM transactions
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    /// Get all transactions for a specific user.
    pub async fn get_transactions_by_user_id(&self, user_id: i64) -> Result<Vec<UserTransaction>> {
        let transactions = sqlx::query_as!(
            UserTransaction,
            r#"
            SELECT 
                id as `id: u32`,
                user_id as `user_id: u32`,
                plan_id as `plan_id: u32`,
                stripe_payment_intent_id as `stripe_payment_intent_id: String`,
                amount as `amount: BigDecimal`,
                status as `status: UserTransactionStatus`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`
            FROM transactions
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    pub async fn get_transaction_by_id(&self, id: i64) -> Result<Option<UserTransaction>> {
        let transaction = sqlx::query_as!(
            UserTransaction,
            r#"
            SELECT
                id as `id: u32`,
                user_id as `user_id: u32`,
                plan_id as `plan_id: u32`,
                stripe_payment_intent_id as `stripe_payment_intent_id: String`,
                amount as `amount: BigDecimal`,
                status as `status: UserTransactionStatus`,
                created_at as `created_at: DateTime<Utc>`,
                updated_at as `updated_at: DateTime<Utc>`
            FROM transactions
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    /// Change the status of a transaction.
    pub async fn change_transaction_status(
        &self,
        transaction_id: i64,
        status: UserTransactionStatus,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE transactions
            SET status = ?
            WHERE id = ?
            "#,
            status as i16,
            transaction_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
