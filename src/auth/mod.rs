use crate::errors::AppError;

pub trait AuthProvider {
    fn get_token(&self) -> Result<String, AppError>;
    fn set_token(&self, token: &str) -> Result<(), AppError>;
}

impl<T: AuthProvider + ?Sized> AuthProvider for Box<T> {
    fn get_token(&self) -> Result<String, AppError> {
        self.as_ref().get_token()
    }

    fn set_token(&self, token: &str) -> Result<(), AppError> {
        self.as_ref().set_token(token)
    }
}

pub mod env;
pub mod keyring;
pub mod token;
