use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use rand::rngs::OsRng;
use rpassword::read_password;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use std::io::Write;

#[derive(Parser)]
#[clap(name = "User Management CLI", version = "1.0", author = "Your Name")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Create a new user
    CreateUser,
    /// Promote a user to admin
    PromoteUser { email: String },
    /// Demote a user from admin
    DemoteUser { email: String },
    /// List all users
    ListUsers,
    /// Change a user's password
    ChangePassword { email: String },
}

async fn create_user(
    pool: &MySqlPool,
    // email: &str,
    // password: &str,
    // admin: bool,
) -> Result<(), sqlx::Error> {
    // request user input
    println!("Enter user email:");
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).unwrap();
    let email = email.trim();

    println!("Enter user password:");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    let password = password.trim();

    println!("Is the user an admin? (y/n)");
    let mut admin_input = String::new();
    std::io::stdin().read_line(&mut admin_input).unwrap();
    let admin = admin_input.trim().to_lowercase() == "y";

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

async fn demote_user(pool: &MySqlPool, email: &str) -> Result<(), sqlx::Error> {
    let result = sqlx::query!("UPDATE users SET is_admin = FALSE WHERE email = ?", email)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        println!("No user found with the given email.");
    } else {
        println!("User demoted from admin successfully.");
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
        Commands::CreateUser => {
            create_user(&pool).await.unwrap();
        }
        Commands::PromoteUser { email } => {
            promote_user(&pool, email).await.unwrap();
        }
        Commands::DemoteUser { email } => {
            demote_user(&pool, email).await.unwrap();
        }
        Commands::ListUsers => {
            let users = sqlx::query!("SELECT email, is_admin FROM users")
                .fetch_all(&pool)
                .await
                .expect("Failed to fetch users");

            println!("{:<30} {:<10}", "Email", "Admin");
            println!("{:<30} {:<10}", "-----", "-----");
            for user in users {
                println!("{:<30} {:<10}", user.email, user.is_admin);
            }
        }
        Commands::ChangePassword { email } => {
            println!("Enter new password:");
            std::io::stdout().flush().unwrap();
            let password = read_password().unwrap();
            let password = password.trim();

            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .unwrap()
                .to_string();

            sqlx::query!(
                "UPDATE users SET password_hash = ? WHERE email = ?",
                password_hash,
                email
            )
            .execute(&pool)
            .await
            .expect("Failed to update password");

            println!("Password updated successfully.");
        }
    }
}
