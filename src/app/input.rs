use std::sync::Mutex;
use termion::{
    event::Key,
    input::{Keys, TermRead},
    AsyncReader,
};
use tokio::{task, time, time::Duration};

pub struct Input {
    keys: Mutex<Keys<AsyncReader>>,
}

impl Input {
    pub fn new() -> anyhow::Result<Self> {
        let keys = termion::async_stdin().keys();

        Ok(Input {
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
