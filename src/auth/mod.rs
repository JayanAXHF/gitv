use crate::errors::AppError;

pub trait AuthProvider {
    fn get_token(&self) -> Result<String, AppError>;
}

pub mod keyring;
pub mod token;
