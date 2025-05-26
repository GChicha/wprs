use std::collections::HashMap;

use crossbeam_channel::Sender;
use tracing::{error, info};
use zbus::{interface, zvariant::Value};

use crate::{
    channel_utils::DiscardingSender,
    serialization::{ForwardedNotification, Request, SendType},
};

pub struct Notifications {
    // stores the remote id mapped to internal id
    id_mapper: HashMap<u32, u32>,
    sender: DiscardingSender<Sender<SendType<Request>>>,
}

impl Notifications {
    pub fn new(sender: DiscardingSender<Sender<SendType<Request>>>) -> Self {
        Self {
            sender,
            id_mapper: HashMap::default(),
        }
    }
}

#[interface(
    name = "org.freedesktop.Notifications",
    proxy(
        gen_blocking = false,
        default_path = "/org/freedesktop/Notifications",
        default_service = "org.freedesktop.Notifications",
    )
)]
impl Notifications {
    /// CloseNotification method
    pub fn close_notification(&self, id: u32) {}

    pub fn get_capabilities(&self) -> Vec<String> {
        vec![
            "actions".to_string(),
            "body".to_string(),
            "icon-static".to_string(),
            "persistence".to_string(),
        ]
    }

    pub fn get_server_information(&self) -> (String, String, String, String) {
        (
            "wprs".to_string(),
            "wprs".to_string(),
            "0.0.1".to_string(),
            "1.3".to_string(),
        )
    }

    /// Notify method
    #[allow(clippy::too_many_arguments)]
    pub fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        _hints: HashMap<&str, Value<'_>>,
        expire_timeout: i32,
    ) -> u32 {
        if let Err(err) = self.sender.send(SendType::Object(Request::Notification(
            ForwardedNotification {
                app_name: app_name.to_string(),
                replaces_id,
                app_icon: app_icon.to_string(),
                summary: summary.to_string(),
                body: body.to_string(),
                actions: actions.into_iter().map(String::from).collect(),
                expire_timeout,
            },
        ))) {
            error!("failed to forward notification to client {:?}", err);
        };
        info!("app {} summary {}", app_name, summary);
        1
    }
}
