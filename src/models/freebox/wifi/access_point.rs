use serde::Deserialize;
use crate::app::{ResponseResult, SuccessResponse};
use crate::client::HttpClient;
use crate::models::freebox::configuration::lan::LanHost;
use crate::models::freebox::wifi::global::{ExpectedPhyBand};

#[derive(Debug, Clone, Deserialize)]
pub enum WifiApStatusState {
    #[serde(rename = "scanning")]
    Scanning,
    #[serde(rename = "no_param")]
    NoParameters,
    #[serde(rename = "bad_param")]
    BadParameters,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "disabled_planning")]
    DisabledPlanning,
    #[serde(rename = "no_active_bss")]
    NoActiveBss,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "acs")]
    Acs,
    #[serde(rename = "ht_scan")]
    HtScan,
    #[serde(rename = "dfs")]
    Dfs,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApCapabilities {
    #[serde(rename = "2d4g")]
    pub b2d4g: i32,
    #[serde(rename = "5g")]
    pub b5g: i32,
    #[serde(rename = "6g")]
    pub b6g: i32,
    #[serde(rename = "60g")]
    pub b60g: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiStationState {
    #[serde(rename = "associated")]
    Associated,
    #[serde(rename = "authenticated")]
    Authenticated,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiAp {
    pub id: i32,
    pub name: String,
    pub status: WifiApStatus,
    pub capabilities: WifiApCapabilities,
    pub config: WifiApConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApBody {
    pub config: WifiApConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApStatus {
    pub state: WifiApStatusState,
    pub channel_width: i32,
    pub primary_channel: i32,
    pub secondary_channel: i32,
    pub dfs_cac_remaining_time: i32,
    pub dfs_disabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApHtConfig {
    pub ac_enabled: bool,
    pub ht_enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApHeConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApConfig {
    pub band: ExpectedPhyBand,
    pub channel_width: i32,
    pub primary_channel: i32,
    pub secondary_channel: i32,
    pub dfs_enabled: bool,
    pub ht: WifiApHtConfig,
    pub he: WifiApHeConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApConfigBody {
    pub band: Option<ExpectedPhyBand>,
    pub channel_width: Option<i32>,
    pub primary_channel: Option<i32>,
    pub secondary_channel: Option<i32>,
    pub dfs_enabled: Option<bool>,
    pub ht: Option<WifiApHtConfig>,
    pub he: Option<WifiApHeConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiApChannelSurveyData {
    pub timestamp: i32,
    pub busy_percent: i32,
    pub tx_percent: i32,
    pub rx_percent: i32,
    pub rx_bss_percent: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiAllowedComb {
    pub band: ExpectedPhyBand,
    pub channel_width: String,
    pub need_dfs: bool,
    pub dfs_cac_time: i32,
    pub primary: i32,
    pub secondary: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiStation {
    pub id: String,
    pub mac: String,
    pub bssid: String,
    pub hostname: String,
    pub host: LanHost,
    pub state: WifiStationState,
    pub inactive: i32,
    pub conn_duration: i32,
    pub rx_bytes: i32,
    pub tx_bytes: i32,
    pub tx_rate: i32,
    pub rx_rate: i32,
    pub signal: i32,
    pub flags: WifiStationFlags,
    pub last_rx: WifiStationStats,
    pub last_tx: WifiStationStats
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiStationFlags {
    pub legacy: bool,
    pub ht: bool,
    pub vht: bool,
    pub he: bool,
    pub authorized: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiStationStats {
    pub bitrate: i64,
    pub mcs: i32,
    pub vht_mcs: i32,
    pub width: String,
    pub shortgi: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiChannelUsage {
    pub channel: i32,
    pub band: ExpectedPhyBand,
    pub noise_level: i32,
    pub rx_busy_percent: i32,
}

pub type ListWifiChannelUsage = ResponseResult<Vec<WifiChannelUsage>>;

pub type GetWifiApResponse = ResponseResult<WifiAp>;
pub type UpdateWifiApResponse = ResponseResult<WifiAp>;
pub type ListWifiApResponse = ResponseResult<Vec<WifiAp>>;

pub type GetWifiAllowedCombResponse = ResponseResult<Vec<WifiAllowedComb>>;

pub type GetWifiStationsResponse = ResponseResult<WifiStation>;
pub type ListWifiStationsResponse = ResponseResult<Vec<WifiStation>>;

pub type ListWifiApChannelSurveyResponse = ResponseResult<Vec<WifiApChannelSurveyData>>;

pub type RestartWifiApResponse = SuccessResponse;

pub type GetWifiApDefaultConfigResponse = ResponseResult<WifiApConfig>;

pub trait WifiApCalls<T: HttpClient> {
    async fn list_wifi_access_points(&self, client: &T, session: &str) -> Result<ListWifiApResponse, T::Error>;
    async fn get_wifi_access_point(&self, client: &T, session: &str, ap_id: &str) -> Result<GetWifiApResponse, T::Error>;
    async fn update_wifi_access_point(&self, client: &T, session: &str, ap_id: &str, body: WifiApBody) -> Result<GetWifiApResponse, T::Error>;
    async fn get_wifi_access_point_allowed_comb(&self, client: &T, session: &str, ap_id: &str) -> Result<GetWifiAllowedCombResponse, T::Error>;
    async fn list_wifi_access_point_stations(&self, client: &T, session: &str, ap_id: &str) -> Result<ListWifiStationsResponse, T::Error>;
    async fn get_wifi_access_point_station(&self, client: &T, session: &str, ap_id: &str, station_mac: &str) -> Result<GetWifiStationsResponse, T::Error>;
    async fn get_wifi_access_point_channel_survey_history(&self, client: &T, session: &str, ap_id: &str, timestamp: i32) -> Result<ListWifiApChannelSurveyResponse, T::Error>;
    async fn restart_wifi_access_point(&self, client: &T, session: &str, ap_id: &str) -> Result<RestartWifiApResponse, T::Error>;
}