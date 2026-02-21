mod error;
pub use error::*;
mod config;
mod listener;
mod notification;

use crate::{config::Config, listener::Listener};

#[tokio::main]
async fn main() -> crate::Result<()> {
    let (_listener, mut rx) = Listener::new().await?;
    let config = Config::try_get()?;
    let sounds = config.load_sounds()?;

    while let Some(notif) = rx.recv().await {
        dbg!(notif);
    }

    Ok(())
}
