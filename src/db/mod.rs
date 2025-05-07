//! # DB Module
//!
//! This module abstracts the database layer of the application, allowing for easy access to various repositories.
//!
//! Each submodule represents a different repository, which is responsible for interacting with a specific table in the database.

mod announcement_repository;
mod plan_repository;
mod transaction_repository;
mod user_repository;
mod verification_repository;

// Re-export all repositories for convenience
pub use announcement_repository::*;
pub use plan_repository::*;
pub use transaction_repository::*;
pub use user_repository::*;
pub use verification_repository::*;
