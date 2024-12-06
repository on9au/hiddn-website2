use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use rand::rngs::OsRng;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;

#[derive(Parser)]
#[clap(name = "User Management CLI", version = "1.0", author = "Your Name")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new user
    CreateUser {
        email: String,
        password: String,
        #[clap(short, long)]
        admin: bool,
    },
    /// Promote a user to admin
    PromoteUser { email: String },
}

async fn create_user(
    pool: &MySqlPool,
    email: &str,
    password: &str,
    admin: bool,
) -> Result<(), sqlx::Error> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    sqlx::query!(
        "INSERT INTO users (email, password_hash, is_admin, created_at, updated_at) VALUES (?, ?, ?, NOW(), NOW())",
        email,
        password_hash,
        admin
    )
    .execute(pool)
    .await?;

    println!("User created successfully.");
    Ok(())
}

async fn promote_user(pool: &MySqlPool, email: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query!("UPDATE users SET is_admin = TRUE WHERE email = ?", email)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        println!("No user found with the given email.");
    } else {
        println!("User promoted to admin successfully.");
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateUser {
            email,
            password,
            admin,
        } => {
            create_user(&pool, email, password, *admin).await.unwrap();
        }
        Commands::PromoteUser { email } => {
            promote_user(&pool, email).await.unwrap();
        }
    }
}
