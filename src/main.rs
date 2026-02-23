mod error;
use std::io::Cursor;

pub use error::*;
use rodio::MixerDeviceSink;
use tokio::sync::mpsc;
use zbus::Connection;
mod config;
mod listener;
mod notification;

use crate::{config::Config, listener::watch_notifications, notification::Urgency};

#[tokio::main]
async fn main() -> crate::Result<()> {
    // Listen to incoming notifications
    let (tx, mut rx) = mpsc::channel(16);
    let connection = Connection::session().await?;
    tokio::spawn(watch_notifications(connection, tx));

    let config = Config::try_get()?;
    let sounds = config.load_sounds()?;
    let urgent_sound = sounds
        .iter()
        .find(|s| s.key.app_name == "notify_sound_urgent");

    let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("Couldn't open default device");

    while let Some(notif) = rx.recv().await {
        if let Urgency::Urgent = notif.urgency()
            && let Some(urgent_sound) = urgent_sound
        {
            play(&sink, urgent_sound.bytes);
        } else {
            for sound in &sounds {
                let origin_name_matches = match notif.origin_name() {
                    Some(origin_name) => sound.app_regex.is_match(origin_name),
                    None => false,
                };

                if origin_name_matches || sound.app_regex.is_match(&notif.app_name) {
                    play(&sink, sound.bytes);

                    break;
                }
            }
        }
    }

    Ok(())
}

fn play(sink: &MixerDeviceSink, bytes: &'static [u8]) {
    let cursor = Cursor::new(bytes);
    let source = rodio::Decoder::new(cursor).expect("Failed to decode sound!");

    sink.mixer().add(source);
}
