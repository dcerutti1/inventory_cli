//Creates sessions

use chrono::{Utc};
use dialoguer::Confirm;
use dialoguer::console::Term;
use sqlx::{SqlitePool};
use crate::functions::all_errors::MyError;

use auth::login::User;
use crate::auth;
use crate::auth::login::Authenticate;

pub async fn create_session(pool: &SqlitePool, user_id: i32, session_id: String, create_at: String, expired_at: String) -> Result<(),MyError> {
    
    sqlx::query("INSERT INTO sessions (session_id, user_id, created_at, expired_at) VALUES (?, ?, ?, ?)")
        .bind(&session_id)
        .bind(&user_id)
        .bind(&create_at)
        .bind(&expired_at)
        .execute(pool)
        .await?;
    
    Ok(())
    
}

pub async fn session_check() -> Result<(), MyError> {
    let pool = SqlitePool::connect("./Database/prod.db").await?;

    // Fetch the session and its expiration time
    let row = sqlx::query("SELECT 1 FROM sessions WHERE expired_at > CURRENT_TIMESTAMP LIMIT 1")
        .fetch_optional(&pool)
        .await?;

    match row {
        Some(_) => {
            Ok(())
        }
        None => {
            println!("Session expired or not found. Please log in.");
            
            let current_time = Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();
            
             sqlx::query("DELETE FROM sessions WHERE expired_at < ?")
                .bind(current_time)
                .execute(&pool)
                .await?;

            let _ = Confirm::new()
                .with_prompt("Press Enter to continue...")
                .default(true)
                .interact()?;
            
            let term = Term::stdout();
            term.clear_last_lines(2);
            
            User::login().await?; 
            Ok(())
        }
    }
}
