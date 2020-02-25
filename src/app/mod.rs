mod input;
mod ui;

use input::Input;
use termion::event::Key;
use tokio::{
    select,
    stream::StreamExt,
    time::{self, Duration},
};
use ui::UI;

pub struct App {}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(App {})
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut ticker = time::interval(Duration::from_secs(1));

        let input = Input::new()?;
        let mut ui = UI::new()?;

        loop {
            select! {
                _ = ticker.next() => {
                    ui.render()?;
                }
                key_result = input.read() => {
                    let key = key_result?;

                    if let Key::Ctrl('c') = key {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
