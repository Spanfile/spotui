mod app;

use app::App;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    app.run().await?;
    Ok(())
}
