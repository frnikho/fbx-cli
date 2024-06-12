/*use serde::{Deserialize, Serialize};
use crate::app::{ResponseResult, SuccessResponse};
use crate::models::freebox::lan::LanHost;

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
pub struct WifiGlobalConfig {
    pub enabled: bool,
    pub mac_filter_state: MacFilterState,
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
pub struct WifiGlobalState {
    pub state: WifiGlobalStateKind,
    pub expected_phys: Vec<ExpectedPhy>,
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

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedPhy {
    pub band: ExpectedPhyBand,
    pub phy_id: i32,
    pub detected: bool,
}

#[derive(Debug, Clone, Deserialize)]
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
pub struct WifiAp {
    pub id: i32,
    pub name: String,
    pub status: WifiApStatus,
    pub capabilities: WifiApCapabilities,
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
pub enum WifiStationState {
    #[serde(rename = "associated")]
    Associated,
    #[serde(rename = "authenticated")]
    Authenticated,
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
pub struct WifiBss {
    pub id: i32,
    pub phy_id: String,
    pub status: WifiBssStatus,
    pub use_shared_params: bool,
    pub config: WifiBssConfig,
    pub bss_params: WifiBssConfig,
    pub shared_bss_params: WifiBssConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBssStatus {
    pub state: WifiBssStatusState,
    pub sta_count: i32,
    pub authorized_sta_count: i32,
    pub custom_key_ssid: String,
    pub is_main_bss: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiBssStatusState {
    #[serde(rename = "phy_stopped")]
    ApStopped,
    #[serde(rename = "no_param")]
    NoParameters,
    #[serde(rename = "bad_param")]
    BadParameters,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBssConfig {
    pub enabled: bool,
    pub use_default_config: bool,
    pub ssid: String,
    pub hide_ssid: String,
    pub encryption: WifiBssConfigEncryption,
    pub key: String,
    pub eapol_version: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiBssConfigEncryption {
    #[serde(rename = "wep")]
    Wep,
    #[serde(rename = "wpa_psk_auto")]
    WpaPskAuto,
    #[serde(rename = "wpa_psk_tkip")]
    WpaPskTkip,
    #[serde(rename = "wpa_psk_ccmp")]
    WpaPskCcmp,
    #[serde(rename = "wpa12_psk_auto")]
    Wpa12PskAuto,
    #[serde(rename = "wpa2_psk_auto")]
    Wpa2PskAuto,
    #[serde(rename = "wpa2_psk_tkip")]
    Wpa2PskTkip,
    #[serde(rename = "wpa2_psk_ccmp")]
    Wap2PskCcmp,
    #[serde(rename = "wpa23_psk_ccmp")]
    Wpa23PskCCmp,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct WifiChannelUsage {
    pub channel: i32,
    pub band: ExpectedPhyBand,
    pub noise_level: i32,
    pub rx_busy_percent: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiPlanning {
    pub use_planning: bool,
    pub resolution: i32,
    pub mapping: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiMacFilter {
    pub id: String,
    pub mac: String,
    pub comment: String,
    pub r#type: WifiMacFilterType,
    pub hostname: String,
    pub host: Option<LanHost>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiMacFilterType {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiDiag {
    pub aps: Vec<WifiDiagItem>,
    pub bssid: Vec<WifiDiagItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiDiagItem {
    pub ap_id: i32,
    pub bssid: String,
    pub code: WifiDiagItemCode,
    pub severity: WifiDiagItemCodeSeverity,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiDiagItemCode {
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
pub enum WifiDiagItemCodeSeverity {
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "major")]
    Major,
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

#[derive(Debug, Clone, Deserialize)]
pub struct WifiCustomKeyHost {
    pub hostname: String,
    pub host: Option<LanHost>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiCustomKeyParams {
    pub description: String,
    pub key: String,
    pub max_use_count: i32,
    pub duration: i32,
    pub access_type: WifiCustomKeyParamsAccessType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiCustomKey {
    pub id: i32,
    pub remaining: i32,
    pub params: WifiCustomKeyParams,
    pub users: Vec<WifiCustomKeyHost>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum WifiCustomKeyParamsAccessType {
    Full,
    NetOnly,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandbyConfig {
    pub use_planning: bool,
    pub planning_mode: StandbyStateMode,
    pub resolution: i32,
    pub mapping: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum StandbyStateMode {
    WifiOff,
    Standby
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandbyStatus {
    pub use_planning: bool,
    pub planning_mode: StandbyStateMode,
    pub next_change: i32,
    pub available_planning_modes: Vec<StandbyStateMode>,
}

pub type GetWifiGlobalConfigResponse = ResponseResult<WifiGlobalConfig>;
pub type UpdateWifiGlobalConfigResponse = ResponseResult<WifiGlobalConfig>;

pub type GetWifiGlobalStateResponse = ResponseResult<WifiGlobalState>;

pub type GetWifiApResponse = ResponseResult<WifiAp>;
pub type UpdateWifiApResponse = ResponseResult<WifiAp>;
pub type ListWifiApResponse = ResponseResult<Vec<WifiAp>>;

pub type GetWifiAllowedCombResponse = ResponseResult<Vec<WifiAllowedComb>>;

pub type GetWifiStationsResponse = ResponseResult<WifiStation>;
pub type ListWifiStationsResponse = ResponseResult<Vec<WifiStation>>;

pub type ListWifiApChannelSurveyResponse = ResponseResult<Vec<WifiApChannelSurveyData>>;

pub type RestartWifiApResponse = SuccessResponse;

pub type GetWifiBss = ResponseResult<WifiBss>;
pub type ListWifiBss = ResponseResult<Vec<WifiBss>>;
pub type UpdateWifiBss = ResponseResult<WifiBss>;

pub type ListWifiNeighbors = ResponseResult<Vec<WifiNeighbor>>;
pub type ListWifiChannelUsage = ResponseResult<Vec<WifiChannelUsage>>;
pub type StartWifiScan = SuccessResponse;

pub type GetWifiMacFilter = ResponseResult<WifiMacFilter>;
pub type ListWifiMacFilter = ResponseResult<Vec<WifiMacFilter>>;
pub type UpdateWifiMacFilter = ResponseResult<WifiMacFilter>;
pub type DeleteWifiMacFilter = SuccessResponse;
pub type CreateWifiMacFilter = ResponseResult<WifiMacFilter>;

pub type ResetWifiGlobalResponse = SuccessResponse;
pub type GetWifiApDefaultConfigResponse = ResponseResult<WifiApConfig>;
pub type GetWifiBssDefaultConfigResponse = ResponseResult<WifiBssConfig>;

pub type ListGlobalWifiDiagResponse = ResponseResult<Vec<WifiDiagItem>>;
pub type GetGlobalWifiDiagResponse = ResponseResult<WifiDiag>;

pub type ListWpsSessions = ResponseResult<Vec<WifiWpsSession>>;
pub type ClearAllWpsSessions = SuccessResponse;

pub type ListWifiCustomKeys = ResponseResult<Vec<WifiCustomKey>>;
pub type GetWifiCustomKey = ResponseResult<WifiCustomKey>;
pub type DeleteWifiCustomKey = SuccessResponse;
pub type CreateWifiCustomKey = ResponseResult<WifiCustomKey>;*/