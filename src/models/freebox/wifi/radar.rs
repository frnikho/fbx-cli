use serde::Deserialize;
use crate::app::{ResponseResult, SuccessResponse};
use crate::client::HttpClient;
use crate::models::freebox::wifi::access_point::ListWifiChannelUsage;
use crate::models::freebox::wifi::global::{ExpectedPhyBand, ResetWifiGlobalResponse};

#[derive(Debug, Clone, Deserialize)]
pub struct WifiNeighbor {
    pub bssid: String,
    pub ssid: String,
    pub band: ExpectedPhyBand,
    pub channel_width: i32,
    pub channel: i32,
    pub secondary_channel: i32,
    pub signal: i32,
    pub capabilities: WifiNeighborCap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiNeighborCap {
    pub legacy: bool,
    pub ht: bool,
    pub vht: bool,
}

pub type ListWifiNeighbors = ResponseResult<Vec<WifiNeighbor>>;
pub type StartWifiScan = SuccessResponse;

pub trait WifiRadarCalls<T: HttpClient> {
    async fn list_wifi_neighbors(&self, client: &T, session: &str, ap_id: &str) -> Result<ListWifiNeighbors, T::Error>;
    async fn list_wifi_channel_usage(&self, client: &T, session: &str, api_id: &str) -> Result<ListWifiChannelUsage, T::Error>;
    async fn refresh_wifi_neighbors_scan(&self, client: &T, session: &str, ap_id: &str) -> Result<StartWifiScan, T::Error>;
    async fn reset_wifi_global(&self, client: &T, session: &str) -> Result<ResetWifiGlobalResponse, T::Error>;
}