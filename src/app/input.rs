use anyhow::anyhow;
use std::{
    io::{self, Stdout, Write},
    sync::{Arc, Mutex},
};
use termion::{
    event::Key,
    input::{Keys, TermRead},
    raw::{IntoRawMode, RawTerminal},
    AsyncReader,
};
use tokio::{task, time, time::Duration};

pub struct Input {
    stdout: Arc<RawTerminal<Stdout>>,
    keys: Mutex<Keys<AsyncReader>>,
}

impl Input {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        let keys = termion::async_stdin().keys();

        Ok(Input {
            stdout: Arc::new(stdout),
            keys: Mutex::new(keys),
        })
    }

    pub async fn read(&self) -> anyhow::Result<Key> {
        loop {
            if let Some(key_result) = self.keys.lock().unwrap().next() {
                let key = key_result?;
                break Ok(key);
            } else {
                time::delay_for(Duration::from_millis(1)).await;
                task::yield_now().await;
                continue;
            }
        }
    }
}
