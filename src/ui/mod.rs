use crate::errors::AppError;
use crate::github::GithubClient;

pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&mut self, _github: &GithubClient) -> Result<(), AppError> {
        Ok(())
    }
}

pub mod components;
pub mod layout;
pub mod theme;
