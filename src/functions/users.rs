
use std::sync::Arc;
use sqlx::{sqlite::SqlitePool, Error, FromRow, Row};
use async_trait::async_trait;
use bcrypt::{hash, DEFAULT_COST};
use bcrypt::BcryptError;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use thiserror::Error;
use tabled::{Table, Tabled};
use tabled::settings::Style;

pub struct Database {
    connection: Arc<SqlitePool>,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Tabled, FromRow)]
struct UserRow {
    id: i32,
    username: String,
    role: String,
}

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Database error: {0}")]
    Database(#[from] Error),
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("IO error: {0}")]
    Dialoger(#[from] dialoguer::Error),
}

#[async_trait]
pub trait UserActions {
    async fn create_user(&self) -> Result<() , MyError>;
    async fn delete_user(&self) -> Result<(), MyError>;
    async fn list_users(&self) -> Result<(), MyError>;
}

impl Database {
    pub async fn new() -> Result<Self, MyError> {
        let pool = SqlitePool::connect("./Database/prod.db").await?;
        Ok(Self { connection: Arc::new(pool) })
    }
}

#[async_trait]
impl UserActions for Database {
    async fn create_user(&self) -> Result<(), MyError> {
        loop {
            let username: String = Input::new()
                .with_prompt("Enter username")
                .interact_text()?;

            let password: String = Input::new()
                .with_prompt("Enter password")
                .interact_text()?;
            let hashed_password = hash(&password, DEFAULT_COST)?;

            let role_selection = vec!["Admin", "Supervisor", "Worker", "User"];
            let selected_index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select one.")
                .items(&role_selection)
                .interact()?;

            let role = role_selection[selected_index].to_string();


            sqlx::query("INSERT INTO users (username, password, role) VALUES (?, ?, ?)")
                .bind(username)
                .bind(hashed_password)
                .bind(role)
                .execute(&*self.connection)
                .await?;
            let input: String = Input::new()
                .with_prompt("done?: Y/N")
                .interact_text()?;

            if input.to_uppercase() == "Y"{
                break
            }
        }
        Ok(())
    }

    async fn delete_user(&self) -> Result<(), MyError> {
        loop {
            let id: i32 = Input::new()
                .with_prompt("Enter ID")
                .interact_text()?;

            sqlx::query("DELETE FROM users WHERE id = ?")
                .bind(id)
                .execute(&*self.connection)
                .await?;
            
           
            let input: String = Input::new()
                .with_prompt("done?: Y/N")
                .interact_text()?;
            
            if input.to_uppercase() == "Y"{
                break
            }
            
        }
        Ok(())
    }

    async fn list_users(&self) -> Result<(), MyError> {
        loop {
            let users: Vec<UserRow> = sqlx::query_as::<_, UserRow>("SELECT id, username, role FROM users")
                .fetch_all(&*self.connection)
                .await?;

            if users.is_empty() {
                println!("==========No users found==========");
            } else {
                let mut table = Table::new(&users); // Create the table
                let new = table.with(Style::modern());
                println!("{}", new)
            }

            let input: String = Input::new()
                .with_prompt("done?: Y/N")
                .interact_text()?;

            if input.to_uppercase() == "Y"{
                break
            }
        }

        Ok(())
    }
}
