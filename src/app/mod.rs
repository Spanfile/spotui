mod input;

use async_std::{prelude::*, stream, stream::Stream};
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
};
use std::time::Duration;

pub struct App {}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(App {})
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let ticker = tick_stream();

        pin_mut!(ticker);

        loop {
            select! {
                () = ticker => {
                    println!("tick");
                }
            }
        }

        Ok(())
    }
}

async fn tick_stream() -> impl Stream<Item = ()> {
    stream::interval(Duration::from_secs(1)).fuse()
}
