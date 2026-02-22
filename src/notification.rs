use std::collections::HashMap;

use serde::Deserialize;
use zbus::zvariant::{self, Type};

#[allow(dead_code)]
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

/* Urgency */
#[derive(Debug, Default, Clone)]
pub enum Urgency {
    Low,
    #[default]
    Normal,
    Urgent,
}

impl From<u64> for Urgency {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Normal,
            2 => Self::Urgent,
            _ => Self::default(),
        }
    }
}

impl Notification {
    /// Get the notification's urgency level. this is used to play a custom sound for urgent
    /// notifications, for example.
    pub fn urgency(&self) -> Urgency {
        self.hints
            .get("urgency")
            .and_then(|v| v.try_into().ok())
            .map(|v: u8| Urgency::from(u64::from(v)))
            .unwrap_or_default()
    }

    /// Get the "Origin Name" of the notification. This is used to play a custom sound for a
    /// notification from a specific website from chromium, for example.
    pub fn origin_name(&self) -> Option<&str> {
        self.hints
            .get("x-kde-origin-name")
            .and_then(|v| v.try_into().ok())
    }
}
