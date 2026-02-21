use std::collections::HashMap;

use futures::StreamExt;
use serde::Deserialize;
use tokio::sync::mpsc;
use zbus::{
    Connection, MatchRule,
    zvariant::{self, Type},
};

#[derive(Debug, Deserialize, Type)]
pub struct Notification {
    pub app_name: String,
    pub replaces_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: HashMap<String, zvariant::OwnedValue>,
    pub expire_timeout: i32,
}

impl Notification {
    fn urgency() {}
}

pub struct Listener {
    event_tx: mpsc::Sender<Notification>,
}

impl Listener {
    pub async fn new() -> crate::Result<(Self, mpsc::Receiver<Notification>)> {
        let (event_tx, event_rx) = mpsc::channel(16);
        let connection = Connection::session().await?;

        tokio::spawn(Self::watch(connection, event_tx.clone()));
        Ok((Self { event_tx }, event_rx))
    }

    async fn watch(
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
}
