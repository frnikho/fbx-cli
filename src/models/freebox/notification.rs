use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotificationError {
    #[serde(rename = "noent")]
    NoDevice,
    #[serde(rename = "inval")]
    InvalidParameters,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationTarget {
    pub id: String,
    pub last_use: i32,
    pub kind: String,
    pub name: String,
    pub api_url: String,
    pub message_type: String,
    pub subscriptions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSend {
    pub box_id: String,
    pub device_type: String,
    pub token: String,
    pub device_name: String,
    pub device_id: String,
}
