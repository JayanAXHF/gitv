use crate::errors::AppError;

pub trait AuthProvider {
    fn get_token(&self) -> Result<String, AppError>;
    fn set_token(&self, token: &str) -> Result<(), AppError>;
}

pub mod keyring;
pub mod token;
