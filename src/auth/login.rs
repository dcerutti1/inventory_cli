
use bcrypt::verify;
use dialoguer::Input;
use sqlx::{Row, SqlitePool};
use crate::functions::all_errors::MyError;

pub struct User {
    username: String,
    password: String,
}

pub trait Authenticate {
    async fn login() -> Result<(), MyError>;
}

impl Authenticate for User {
    async fn login() -> Result<(), MyError> {
        // Connect to the SQLite database
        let pool = SqlitePool::connect("./Database/prod.db").await?;

        // Get user input for username and password
        let username: String = Input::new()
            .with_prompt("Enter username")
            .interact_text()?;

        let password: String = Input::new()
            .with_prompt("Enter password")
            .interact_text()?;

        // Fetch the stored password hash from the database
        let row = sqlx::query("SELECT password FROM users WHERE username = ?")
            .bind(&username)
            .fetch_optional(&pool)
            .await?;

        // Check if the user exists
        match row {
            Some(row) => {
                // Retrieve the password hash from the query result
                let stored_hash: String = row.get("password");

                // Verify the entered password against the stored hash
                if verify(&password, &stored_hash).unwrap_or(false) {
                    println!("Login successful!");
                    Ok(()) // Login successful
                } else {
                    Err(MyError::Login("Invalid username or password".to_string()))
                }
            }
            None => Err(MyError::Login("User not found".to_string())),
        }
    }
}