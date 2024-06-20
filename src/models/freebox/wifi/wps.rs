use serde::{Deserialize, Serialize};
use crate::app::{ResponseResult, SuccessResponse};
use crate::client::HttpClient;
use crate::models::freebox::wifi::bss::{WifiBssConfigEncryption, WifiBssStatusState};

#[derive(Debug, Clone, Deserialize)]
pub enum WifiWpsSessionResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "user_canceled")]
    UserCanceled,
    #[serde(rename = "self_canceled")]
    SelfCanceled,
    #[serde(rename = "failed_timeout")]
    Timeout,
    #[serde(rename = "failed_overlap")]
    Overlap,
    #[serde(rename = "failed_unknown")]
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiWpsStartBody {
    pub bssid: String
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiWpsStopBody {
    pub session_id: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiWpsCandidate {
    pub bssid: String,
    pub ssid: String,
    pub bss_uuid: String,
    pub band: String,
    pub encryption: WifiBssConfigEncryption,
    pub wps_enabled: bool,
    pub state: WifiBssStatusState,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiWpsConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiWpsConfigBody {
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiWpsSession {
    pub id: i32,
    pub bss_uuid: String,
    pub ssid: String,
    pub active: bool,
    pub result: WifiWpsSessionResult,
    pub start_date: i32,
    pub end_date: i32,
    pub mac: String,
}

pub type ListWpsSessions = ResponseResult<Vec<WifiWpsSession>>;
pub type ClearAllWpsSessions = SuccessResponse;

pub type WifiWpsConfigResponse = ResponseResult<WifiWpsConfig>;
pub type WifiWpsStartSessionResponse = ResponseResult<i32>;
pub type WifiWpsStopSessionResponse = SuccessResponse;

pub trait WifiWpsCalls<T: HttpClient> {
    async fn get_wifi_wps_config(&self, client: &T, session: &str) -> Result<WifiWpsConfigResponse ,T::Error>;
    async fn update_wifi_wps_config(&self, client: &T, session: &str, body: WifiWpsConfigBody) -> Result<SuccessResponse, T::Error>;
    async fn start_wifi_wps_session(&self, client: &T, session: &str, bssid: &str) -> Result<WifiWpsStartSessionResponse, T::Error>;
    async fn cancel_wifi_wps_session(&self, client: &T, session: &str, wps_session_id: &str) -> Result<WifiWpsStopSessionResponse, T::Error>;
    async fn list_wifi_wps_sessions(&self, client: &T, session: &str) -> Result<ListWpsSessions, T::Error>;
    async fn clear_wifi_wps_sessions(&self, client: &T, session: &str) -> Result<SuccessResponse, T::Error>;
}