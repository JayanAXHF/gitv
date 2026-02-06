use crate::auth::AuthProvider;
use crate::errors::AppError;
use crate::github::GithubClient;
use crate::ui::Ui;

pub struct App {
    github: GithubClient,
    ui: Ui,
}

impl App {
    pub async fn new() -> Result<Self, AppError> {
        let auth = crate::auth::keyring::KeyringAuth::new("issue_me")?;
        let token = auth.get_token().ok();
        let github = GithubClient::new(token)?;
        let ui = Ui::new();
        Ok(Self { github, ui })
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        self.ui.run(&self.github).await
    }
}

pub mod actions;
pub mod commands;
pub mod state;
