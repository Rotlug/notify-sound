use futures::StreamExt;
use tokio::sync::mpsc;
use zbus::{Connection, MatchRule};

use crate::notification::Notification;

/// Monitors `DBus` for incoming notifications and sends them to `tx`
pub async fn watch_notifications(
    connection: Connection,
    event_tx: mpsc::Sender<Notification>,
) -> crate::Result<()> {
    let proxy = zbus::fdo::MonitoringProxy::builder(&connection)
        .destination("org.freedesktop.DBus")?
        .path("/org/freedesktop/DBus")?
        .build()
        .await?;

    let rule = MatchRule::builder()
        .msg_type(zbus::message::Type::MethodCall)
        .interface("org.freedesktop.Notifications")?
        .member("Notify")?
        .path("/org/freedesktop/Notifications")?
        .build();

    proxy.become_monitor(&[rule], 0).await?;

    let mut stream = zbus::MessageStream::from(&connection);

    while let Some(Ok(msg)) = stream.next().await {
        let Ok(body) = msg.body().deserialize::<Notification>() else {
            continue;
        };

        let _ = event_tx.send(body).await;
    }

    Ok(())
}
