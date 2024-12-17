use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::app::{ResponseResult, SuccessResponse};

#[derive(Error, Clone, Debug, Serialize, Deserialize)]
pub enum AirMediaError {
    #[error("No airmedia device with this name in range")]
    #[serde(rename = "unknown_target")]
    UnknownTarget,
    #[error("No airmedia client connected")]
    #[serde(rename = "no_client")]
    NoClient,
    #[error("Unable to update password")]
    #[serde(rename = "set_pass")]
    UpdatePassword,
    #[error("Unable to activate onscreen code")]
    #[serde(rename = "set_onscreen_code")]
    ActivateOnScreenCode,
    #[error("Remote control is unavailable")]
    #[serde(rename = "no_ctrl")]
    RemoteControl,
    #[error("Internal HTTP error")]
    #[serde(rename = "http")]
    InternalHttp,
    #[error("No stream session found")]
    #[serde(rename = "bad_session")]
    NoStreamSession,
    #[error("Invalid airmedia name")]
    #[serde(rename = "bad_name")]
    BadDeviceName,
    #[error("No device with this id")]
    #[serde(rename = "bad_device_id")]
    BadDeviceId,
    #[error("No remote control with this id")]
    #[serde(rename = "bad_remote_id")]
    BadRemoteId,
    #[error("You should try again, another request is still processing")]
    #[serde(rename = "req_in_progress")]
    RequestInProgress,
    #[error("Unable to get slideshow information")]
    #[serde(rename = "fetch")]
    Fetch,
    #[error("No screen available")]
    #[serde(rename = "no_display")]
    NoDisplay,
    #[error("Invalid playback state")]
    #[serde(rename = "playback_state")]
    InvalidPlaybackState,
    #[error("Slideshow is not supported")]
    #[serde(rename = "no_slideshow_srv")]
    SlideshowNotSupported,
    #[error("Internal error")]
    #[serde(rename = "no_mem")]
    InternalError,
    #[error("Unable to read input file")]
    #[serde(rename = "inout_file")]
    UnableReadInputFile,
    #[error("Volume control is not available")]
    #[serde(rename = "no_volume_control")]
    VolumeControlNotAvailable,
    #[error("Error connecting to the airmedia device")]
    #[serde(rename = "connect")]
    Connect,
    #[error("This device requests a password")]
    #[serde(rename = "unauthorized")]
    Unauthorized,
    #[error("The device does not support this format")]
    #[serde(rename = "unsupported_media")]
    UnsupportedMedia,
    #[error("Invalid file type")]
    #[serde(rename = "bad_type")]
    BadFileType,
    #[error("Unimplemented")]
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

#[derive(Clone, Debug, Serialize)]
pub struct AirMediaReceiverRequest {
    pub action: AirMediaReceiverAction,
    pub media_type: AirMediaReceiverMediaType,
    pub password: String,
    pub position: i64,
    pub media: String,
}

#[derive(Clone, Debug, Serialize)]
pub enum AirMediaReceiverAction {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "stop")]
    Stop
}

#[derive(Clone, Debug, Serialize)]
pub enum AirMediaReceiverMediaType {
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "video")]
    Video,
}

pub type GetAirMediaConfigResponse = ResponseResult<AirMediaConfig>;
pub type UpdateAirMediaConfigResponse = ResponseResult<AirMediaConfig>;

pub type ListAirMediaReceiversResponse = ResponseResult<Vec<AirMediaReceiver>>;

pub type InteractAirMediaReceiverResponse = SuccessResponse;

