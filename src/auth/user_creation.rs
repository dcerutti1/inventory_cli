use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use sqlx::SqlitePool;
use bcrypt::{hash, DEFAULT_COST};
use std::error::Error;


pub struct User {
    username: String,
    password: String,
    role: String,
}

pub trait Create{
    async fn create(pool: &SqlitePool) -> Result<User, Box<dyn Error>>;
}

impl Create for User {
    async fn create(pool: &SqlitePool) -> Result<User, Box<dyn Error>> {
         
        let username: String = Input::new()
            .with_prompt("Enter username")
            .interact_text()
            .map_err(|_| "❌ Failed to read input.")?;

        let password: String = Input::new()
            .with_prompt("Enter password")
            .interact_text()
            .map_err(|_| "❌ Failed to read input.")?;
        
        let role_selection = vec!["Admin", "Supervisor","Worker", "User"];
        let selected_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select one.")
            .items(&role_selection)
            .interact()
            .unwrap();

        let role = role_selection[selected_index].to_string();

         let user = User { username, password, role };
          
         create_user(&user, pool).await.map_err(|e|{
             println!("Error creating user: {}", e);
         });

         Ok(user)
     }
    }
    


pub async fn create_user(user: &User, Pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    
    
    let hashed_password = hash(&user.password, DEFAULT_COST).map_err(|e|{
        format!("err hashing: {}", e)
    })?;
    
    
    let query = sqlx::query(" INSERT INTO users (username, password, role) VALUES (?,?,?)")
        .bind(&user.username)
        .bind(hashed_password)
        .bind(&user.role)
        .execute(Pool)
        .await
        .map(|e|{
            format!("error submitting query:")
        })?;
    
    
    Ok(())
}