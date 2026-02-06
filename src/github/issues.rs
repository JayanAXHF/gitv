use crate::errors::AppError;
use crate::models::Issue;

pub async fn list_issues() -> Result<Vec<Issue>, AppError> {
    Ok(Vec::new())
}
