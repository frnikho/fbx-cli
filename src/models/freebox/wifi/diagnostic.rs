use serde::Deserialize;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Debug, Clone, Deserialize)]
pub enum WifiDiagnosticItemCode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "network_disabled")]
    NetworkDisabled,
    #[serde(rename = "network_security")]
    NetworkSecurity,
    #[serde(rename = "network_visibility")]
    NetworkVisibility,
    #[serde(rename = "channel_width")]
    ChannelWidth,
    #[serde(rename = "channel_value")]
    ChannelValue,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiDiagnosticItemCodeSeverity {
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "major")]
    Major,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiDiagnostic {
    pub aps: Vec<WifiDiagnosticItem>,
    pub bssid: Vec<WifiDiagnosticItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiDiagnosticItem {
    pub ap_id: i32,
    pub bssid: String,
    pub code: WifiDiagnosticItemCode,
    pub severity: WifiDiagnosticItemCodeSeverity,
}

pub type ListGlobalWifiDiagnosticResponse = ResponseResult<Vec<WifiDiagnosticItem>>;
pub type GetGlobalWifiDiagnosticResponse = ResponseResult<WifiDiagnostic>;

pub trait WifiDiagnosticCall<T: HttpClient> {
    async fn get_wifi_diagnostic(&self, client: &T, session: &str) -> Result<GetGlobalWifiDiagnosticResponse, T::Error>;
}