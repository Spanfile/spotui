mod input;

use input::Input;
use tokio::{
    select,
    stream::StreamExt,
    time::{self, Duration},
};

pub struct App {}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(App {})
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut ticker = time::interval(Duration::from_secs(1));
        let input = Input::new()?;
        let mut ticks = 0;

        loop {
            select! {
                _ = ticker.next() => {
                    ticks += 1;
                }
                key_result = input.read() => {
                    let key = key_result?;
                    println!("{:?} tick {}", key, ticks);
                }
            }
        }

        // Ok(())
    }
}
