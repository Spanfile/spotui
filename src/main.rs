mod app;

use app::App;
use futures::executor::block_on;

pub fn main() -> anyhow::Result<()> {
    block_on(run())
}

async fn run() -> anyhow::Result<()> {
    let app = App::new()?;
    app.run().await?;
    Ok(())
}
