use crate::errors::AppError;
use crate::github::GithubClient;
use crate::models::Issue;

pub struct Commands;

impl Commands {
    pub async fn fetch_issues(_github: &GithubClient) -> Result<Vec<Issue>, AppError> {
        Ok(Vec::new())
    }
}
