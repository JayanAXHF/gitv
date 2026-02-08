use crate::errors::AppError;

pub struct GithubClient {
    inner: octocrab::Octocrab,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Result<Self, AppError> {
        let mut builder = octocrab::Octocrab::builder();
        if let Some(token) = token {
            builder = builder.personal_token(token);
        }
        let inner = builder.build()?;
        Ok(Self { inner })
    }

    pub fn inner(&self) -> &octocrab::Octocrab {
        &self.inner
    }
}

pub mod issues;
pub mod labels;
pub mod repos;
