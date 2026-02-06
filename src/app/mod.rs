use crate::auth::AuthProvider;
use crate::errors::AppError;
use crate::github::GithubClient;
use crate::ui;
use std::sync::OnceLock;

pub struct App;

pub static GITHUB_CLIENT: OnceLock<GithubClient> = OnceLock::new();

impl App {
    pub async fn new() -> Result<Self, AppError> {
        let auth = crate::auth::keyring::KeyringAuth::new("issue_me")?;
        let token = auth.get_token().ok();
        let github = GithubClient::new(token)?;
        let _ = GITHUB_CLIENT.set(github);
        Ok(Self)
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        ui::run().await
    }
}

pub mod actions;
pub mod commands;
pub mod state;
