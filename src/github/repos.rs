use crate::errors::AppError;
use crate::models::Repo;

pub async fn get_repo() -> Result<Repo, AppError> {
    Err(AppError::NotImplemented)
}
