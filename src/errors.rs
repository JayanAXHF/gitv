use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not implemented")]
    NotImplemented,
    #[error(transparent)]
    Octocrab(#[from] Box<octocrab::Error>),
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
