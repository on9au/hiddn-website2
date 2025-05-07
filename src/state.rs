use std::sync::Arc;

use marzban_api::client::MarzbanAPIClient;
use sqlx::MySqlPool;
use tera::Tera;

use crate::db::{
    AnnouncementRepository, PlanRepository, TransactionRepository, UserRepository,
    VerificationRepository,
};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

impl AppState {
    /// Marzban Client
    pub fn marzban_client(&self) -> &MarzbanAPIClient {
        &self.inner.marzban_client
    }

    /// Stripe Client
    pub fn stripe_client(&self) -> &stripe::Client {
        &self.inner.stripe_client
    }

    /// Tera Template Engine
    pub fn tera(&self) -> &Tera {
        &self.inner.tera
    }

    /// DB >>> User Repository
    pub fn user_repository(&self) -> UserRepository {
        UserRepository::new(self.inner.db_pool.clone())
    }

    /// DB >>> Announcement Repository
    pub fn announcement_repository(&self) -> AnnouncementRepository {
        AnnouncementRepository::new(self.inner.db_pool.clone())
    }

    /// DB >>> Plan Repository
    pub fn plan_repository(&self) -> PlanRepository {
        PlanRepository::new(self.inner.db_pool.clone())
    }

    /// DB >>> Transaction Repository
    pub fn transaction_repository(&self) -> TransactionRepository {
        TransactionRepository::new(self.inner.db_pool.clone())
    }

    /// DB >>> Verification Repository
    pub fn verification_repository(&self) -> VerificationRepository {
        VerificationRepository::new(self.inner.db_pool.clone())
    }

    /// # Warning
    ///
    /// This is **discouraged** for production use.
    ///
    /// Please use the repository methods instead.
    ///
    /// If you are finding yourself needing to use this, consider adding a new method to the repository,
    /// or creating a new repository.
    pub fn raw_db_pool(&self) -> &MySqlPool {
        &self.inner.db_pool
    }
}

pub struct AppStateInner {
    pub db_pool: MySqlPool,
    pub marzban_client: MarzbanAPIClient,
    pub stripe_client: stripe::Client,
    pub tera: Tera,
}

impl From<AppStateInner> for AppState {
    fn from(inner: AppStateInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}
