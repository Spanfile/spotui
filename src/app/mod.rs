mod input;

use input::Input;
use tokio::{
    pin, select,
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

        loop {
            let input_read = input.read();
            pin!(input_read);
            select! {
                _ = ticker.next() => {
                    println!("tick");
                }
                key_result = input_read => {
                    let key = key_result?;
                }
            }
        }

        // Ok(())
    }
}
