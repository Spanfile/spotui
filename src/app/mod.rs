pub struct App {}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(App {})
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
