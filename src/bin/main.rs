use issue_me::{app::App, errors::AppError};

#[tokio::main]
async fn main() -> anyhow::Result<(), AppError> {
    let mut app = App::new().await?;
    app.run().await
}
