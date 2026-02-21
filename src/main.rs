mod error;
pub use error::*;

use crate::listener::Listener;
mod listener;
mod notification;

#[tokio::main]
async fn main() -> crate::Result<()> {
    let (_listener, mut rx) = Listener::new().await?;

    while let Some(notif) = rx.recv().await {
        dbg!(notif);
    }

    Ok(())
}
