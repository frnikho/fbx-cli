use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, Error, Deserialize)]
pub enum PlayerError {
    #[error("Internal error")]
    #[serde(rename = "inval")]
    Internal,
    #[error("invalid parameters")]
    #[serde(rename = "inval")]
    InvalidParameters,
    #[error("no player with this id")]
    #[serde(rename = "noent")]
    NotEntity,
}

#[derive(Debug, Clone, Deserialize)]
pub enum PlayerStatusPowerState {
    #[serde(rename = "standby")]
    Standby,
    #[serde(rename = "running")]
    Running,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub id: i32,
    pub device_name: String,
    pub uid: String,
    pub reachable: bool,
    pub api_version: Option<String>,
    pub api_available: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStatusForegroundApp {
    pub package_id: i32,
    pub curl_url: String,
    pub context: Value,
    pub package: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStatusCapabilities {
    pub play: bool,
    pub pause: bool,
    pub stop: bool,
    pub next: bool,
    pub prev: bool,
    pub record: bool,
    pub record_stop: bool,
    pub seek_forward: bool,
    pub seek_backward: bool,
    pub seek_to: bool,
    pub shuffle: bool,
    pub repeat_all: bool,
    pub repeat_one: bool,
    pub select_stream: bool,
    pub select_audio_track: bool,
    pub select_srt_track: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStatusInformation {
    pub name: String,
    pub last_activity: i32,
    pub capabilities: PlayerStatusCapabilities,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStatus {
    pub power_state: PlayerStatusPowerState,
    pub player: PlayerStatusInformation,
    pub foreground_app: PlayerStatusForegroundApp,
}