use anyhow::anyhow;
use std::{
    io::{self, Stdout, Write},
    sync::Mutex,
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
use tokio::task;

pub struct Input {
    stdout: Mutex<RawTerminal<Stdout>>,
}

impl Input {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        stdout.suspend_raw_mode()?;
        Ok(Input {
            stdout: Mutex::new(stdout),
        })
    }

    pub async fn read(&self) -> anyhow::Result<Key> {
        task::spawn_blocking(move || {
            let mut keys = io::stdin().keys();
            self.activate_raw_mode()?;

            if let Some(key_result) = keys.next() {
                let key = key_result?;
                self.suspend_raw_mode()?;
                Ok(key)
            } else {
                Err(anyhow!("no key"))
            }
        })
        .await?
    }

    pub fn activate_raw_mode(&self) -> anyhow::Result<()> {
        Ok(self.stdout.get_mut()?.activate_raw_mode()?)
    }

    pub fn suspend_raw_mode(&self) -> anyhow::Result<()> {
        Ok(self.stdout.get_mut()?.suspend_raw_mode()?)
    }

    pub fn flush(&self) -> anyhow::Result<()> {
        Ok(self.stdout.get_mut()?.lock().flush()?)
    }
}
