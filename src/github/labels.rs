use crate::errors::AppError;
use crate::models::Label;

pub async fn list_labels() -> Result<Vec<Label>, AppError> {
    Ok(Vec::new())
}
