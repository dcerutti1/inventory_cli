
use bcrypt::verify;
use dialoguer::Input;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use crate::functions::all_errors::MyError;
use auth::sessions::create_session;
use crate::auth;
use chrono::{Duration, Utc};
use dialoguer::console::Term;

pub struct User {
    username: String,
    password: String,
}

pub trait Authenticate {
    async fn login() -> Result<(), MyError>;
}

impl Authenticate for User {
     async fn login() -> Result<(), MyError> {
         let term = Term::stdout();
         
         
        let pool = SqlitePool::connect("./Database/prod.db").await?;

        
        let username: String = Input::new()
            .with_prompt("Enter username")
            .interact_text()?;
         term.clear_last_lines(1);
         
        let password: String = Input::new()
            .with_prompt("Enter password")
            .interact_text()?;
         term.clear_last_lines(1);

        
        let row = sqlx::query("SELECT id, password FROM users WHERE username = ?")
            .bind(&username)
            .fetch_optional(&pool)
            .await?;

        // Check if the user exists
        match row {
            Some(row) => {
                
                let stored_hash: String = row.get("password");

               
                if verify(&password, &stored_hash).unwrap_or(false) {
                    println!("Login successful!");
                    term.clear_last_lines(1);
                    
                    let user_id = row.get("id");


                    let session_id = Uuid::new_v4().to_string(); 

                    let current_time = Utc::now(); // 
                    let expiration_time = current_time + Duration::seconds(30);

                   
                    let current_time_str = current_time.to_string();
                    let expiration_time_str = expiration_time.to_string();
                    
                    
                    
                    create_session(&pool, user_id, session_id, current_time_str, expiration_time_str ).await?;
                    
                    
                    Ok(()) 
                } else {
                    Err(MyError::Login("Invalid username or password".to_string()))
                }
            }
            None => Err(MyError::Login("User not found".to_string())),
        }
    }
    
}