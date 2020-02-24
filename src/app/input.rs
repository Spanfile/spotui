use std::{
    cell::RefCell,
    future::Future,
    io::{self, Stdout, Write},
    rc::Rc,
    task::Poll,
};
use termion::{
    event::Key,
    input::{Keys, TermRead},
    raw::{IntoRawMode, RawTerminal},
};

pub struct Input {
    stdout: Rc<RawTerminal<Stdout>>,
}

pub struct InputRead {
    stdout: Rc<RawTerminal<Stdout>>,
    keys: RefCell<Keys<termion::AsyncReader>>,
}

impl Input {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        stdout.suspend_raw_mode()?;
        Ok(Input {
            stdout: Rc::new(stdout),
        })
    }

    pub async fn read(&self) -> anyhow::Result<InputRead> {
        let stdout = self.stdout.clone();
        let keys = termion::async_stdin().keys();

        Ok(InputRead {
            stdout,
            keys: RefCell::new(keys),
        })
    }
}

impl Future for InputRead {
    type Output = anyhow::Result<Key>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Err(e) = self.stdout.activate_raw_mode() {
            return Poll::Ready(Err(e.into()));
        }

        let result = match self.keys.try_borrow_mut()?.next() {
            Some(key) => Poll::Ready(Ok(key?)),
            None => Poll::Pending,
        };

        match self.stdout.suspend_raw_mode() {
            Err(e) => Poll::Ready(Err(e.into())),
            _ => match self.stdout.lock().flush() {
                Err(e) => Poll::Ready(Err(e.into())),
                _ => result,
            },
        }
    }
}
