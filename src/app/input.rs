use anyhow::anyhow;
use std::{
    io::{self, Stdout, Write},
    sync::Arc,
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
use tokio::task;

pub struct Input {
    stdout: Arc<RawTerminal<Stdout>>,
}

impl Input {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        stdout.activate_raw_mode()?;
        Ok(Input {
            stdout: Arc::new(stdout),
        })
    }

    pub async fn read(&self) -> anyhow::Result<Key> {
        task::spawn_blocking(move || {
            let stdin = io::stdin();
            let mut keys = stdin.lock().keys();
            if let Some(key_result) = keys.next() {
                let key = key_result?;
                Ok(key)
            } else {
                Err(anyhow!("no key"))
            }
        })
        .await?
    }
}
