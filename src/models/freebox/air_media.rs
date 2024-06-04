use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AirMediaError {
    #[serde(rename = "unknown_target")]
    UnknownTarget,
    #[serde(rename = "no_client")]
    NoClient,
    #[serde(rename = "set_pass")]
    UpdatePassword,
    #[serde(rename = "set_onscreen_code")]
    ActivateOnScreenCode,
    #[serde(rename = "no_ctrl")]
    RemoteControl,
    #[serde(rename = "http")]
    InternalHttp,
    #[serde(rename = "bad_session")]
    NoStreamSession,
    #[serde(rename = "bad_name")]
    BadDeviceName,
    #[serde(rename = "bad_device_id")]
    BadDeviceId,
    #[serde(rename = "bad_remote_id")]
    BadRemoteId,
    #[serde(rename = "req_in_progress")]
    RequestInProgress,
    #[serde(rename = "fetch")]
    Fetch,
    #[serde(rename = "no_display")]
    NoDisplay,
    #[serde(rename = "playback_state")]
    InvalidPlaybackState,
    #[serde(rename = "no_slideshow_srv")]
    SlideshowNotSupported,
    #[serde(rename = "no_mem")]
    InternalError,
    #[serde(rename = "inout_file")]
    UnableReadInputFile,
    #[serde(rename = "no_volume_control")]
    VolumeControlNotAvailable,
    #[serde(rename = "connect")]
    Connect,
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[serde(rename = "unsupported_media")]
    UnsupportedMedia,
    #[serde(rename = "bad_type")]
    BadFileType,
    #[serde(rename = "unimplemented")]
    Unimplemented,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AirMediaConfig {
    pub enabled: bool,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AirMediaReceiver {
    pub name: String,
    pub password_protected: bool,
    pub capabilities: Vec<AirMediaReceiverCapabilities>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AirMediaReceiverCapabilities {
    pub photo: bool,
    pub audio: bool,
    pub video: bool,
    pub screen: bool,
}
