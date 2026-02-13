use std::{env, path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::errors::AppError;

pub static PROJECT_NAME: LazyLock<String> =
    LazyLock::new(|| env!("CARGO_CRATE_NAME").to_uppercase().to_string());
pub static DATA_FOLDER: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    env::var(format!("{}_DATA", &*PROJECT_NAME))
        .ok()
        .map(PathBuf::from)
});

pub static LOG_ENV: LazyLock<String> = LazyLock::new(|| format!("{}_LOG_LEVEL", &*PROJECT_NAME));
pub static LOG_FILE: LazyLock<String> = LazyLock::new(|| format!("{}.log", env!("CARGO_PKG_NAME")));

pub(crate) fn get_data_dir() -> PathBuf {
    if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    }
}

pub(crate) fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "jayanaxhf", env!("CARGO_PKG_NAME"))
}

pub fn init() -> Result<(), AppError> {
    //TODO: Add proper directory for logs
    let directory = get_data_dir();
    std::fs::create_dir_all(directory.clone())?;
    let log_path = directory.join(&*LOG_FILE);
    let log_file = std::fs::File::create(log_path)?;

    let file_subscriber = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(true)
        .with_ansi(false);
    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .try_init()?;

    Ok(())
}
