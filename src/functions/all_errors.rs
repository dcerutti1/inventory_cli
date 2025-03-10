use bcrypt::BcryptError;
use log::error;
use sqlx::Error;
use thiserror::Error;
use crate::functions::{all_errors, users};

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Database error: {0}")]
    Database(#[from] Error),
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] BcryptError),
    #[error("dialoguer error: {0}")]
    Dialoger(#[from] dialoguer::Error),
    #[error("An unknown error occurred{0}")]
    Login(String),
    #[error("unknown error: {0}")]
    CustomError(String),
}

impl From<users::MyError> for MyError {
    fn from(err: users::MyError) -> Self {
        MyError::CustomError(err.to_string()) 
    }
}
