use serde::{Deserialize, Serialize};
use crate::app::{ResponseResult, SuccessResponse};
use crate::client::HttpClient;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WifiError {
    #[serde(rename = "inval")]
    InvalidParameter,
    #[serde(rename = "exist")]
    EntryAlreadyExist,
    #[serde(rename = "nospc")]
    MaxEntryReached,
    #[serde(rename = "nodev")]
    InvalidDeviceId,
    #[serde(rename = "noent")]
    InvalidId,
    #[serde(rename = "busy")]
    DeviceBusy,
    #[serde(rename = "inval_band")]
    InvalidWifiBand,
    #[serde(rename = "inval_ssid")]
    InvalidSsid,
    #[serde(rename = "inval_freq")]
    InvalidFrequency,
    #[serde(rename = "inval_cipher")]
    InvalidCipher,
    #[serde(rename = "inval_key_len")]
    InvalidKeyLength,
    #[serde(rename = "inval_key")]
    InvalidKey,
    #[serde(rename = "inval_ht_needs_wmm")]
    WmmMustBeEnabled,
    #[serde(rename = "inval_ac_needs_ht")]
    InvalidConfigNeedHtSupport,
    #[serde(rename = "inval_ac_not_2d4g")]
    InvalidConfigNotSupported,
    #[serde(rename = "inval_wps_needs_ccmp")]
    WpsNeedWpaAesToBeEnabled,
    #[serde(rename = "inval_wps_macfilter")]
    WpsMacFilterEnabled,
    #[serde(rename = "inval_wps_hidden_ssid")]
    VpsHiddenSsid,
    #[serde(rename = "inval_eht_needs_he")]
    AxMustBeEnabled,
    #[serde(rename = "inval_ht_needs_ht")]
    NMustBeEnable,
    #[serde(rename = "inval_ht_needs_vht")]
    AcMustBeEnable,
    #[serde(rename = "inval_6g_needs_he")]
    Invalid6GNeedHe,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MacFilterState {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiGlobalStateKind {
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "disabled_planning")]
    DisabledPlanning,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ExpectedPhyBand {
    #[serde(rename = "2d4g")]
    Band2d4g,
    #[serde(rename = "5g")]
    Band5G,
    #[serde(rename = "6g")]
    Band6G,
    #[serde(rename = "60g")]
    Band60G,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiGlobalConfig {
    pub enabled: bool,
    pub mac_filter_state: MacFilterState,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiGlobalConfigBody {
    pub enabled: Option<bool>,
    pub mac_filter_state: Option<MacFilterState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiGlobalState {
    pub state: WifiGlobalStateKind,
    pub expected_phys: Vec<ExpectedPhy>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedPhy {
    pub band: ExpectedPhyBand,
    pub phy_id: i32,
    pub detected: bool,
}

pub type WifiGlobalConfigResponse = ResponseResult<WifiGlobalConfig>;
pub type GetWifiGlobalStateResponse = ResponseResult<WifiGlobalState>;

pub type ResetWifiGlobalResponse = SuccessResponse;

pub trait WifiCalls<T: HttpClient> {
    async fn get_wifi_config(&self, client: &T, session: &str) -> Result<WifiGlobalConfigResponse, T::Error>;
    async fn update_wifi_config(&self, client: &T, session: &str, body: WifiGlobalConfigBody) -> Result<WifiGlobalConfigResponse, T::Error>;
    async fn get_wifi_status(&self, client: &T, session: &str) -> Result<WifiGlobalState, T::Error>;
    //async fn reset_wifi_bss(&self, client: &T, session: &str, bss_id: &str) -> Result<(), T::Error>;
    //TODO implement other reset
}