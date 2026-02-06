use issue_me::app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new().await?;
    app.run().await
}
