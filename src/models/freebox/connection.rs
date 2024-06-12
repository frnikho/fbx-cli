/*use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnectionError {
    #[serde(rename = "inval")]
    Invalid,
    #[serde(rename = "nodev")]
    NoDevice,
    #[serde(rename = "noent")]
    NoEntity,
    #[serde(rename = "netdown")]
    NetworkDown,
    #[serde(rename = "busy")]
    DeviceBusy,
    #[serde(rename = "invalid_port")]
    InvalidPort,
    #[serde(rename = "insecure_password")]
    InsecurePassword,
    #[serde(rename = "invalid_provider")]
    InvalidProvider,
    #[serde(rename = "invalid_next_hop")]
    InvalidNextHop,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub state: ConnectionStatusState,
    #[serde(rename = "type")]
    pub kind: ConnexionStatusType,
    pub media: ConnexionStatusMedia,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub rate_up: i32,
    pub rate_down: i32,
    pub bandwidth_up: i32,
    pub bandwidth_down: i32,
    pub bytes_up: i32,
    pub bytes_down: i32,
    pub ipv4_port_range: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnectionStatusState {
    #[serde(rename = "going_up")]
    GoingUp,
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "going_down")]
    GoingDown,
    #[serde(rename = "down")]
    Down,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnexionStatusType {
    #[serde(rename = "ethernet")]
    Ethernet,
    #[serde(rename = "rfc2684")]
    RFC2684,
    #[serde(rename = "pppoatm")]
    PPPoA,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnexionStatusMedia {
    #[serde(rename = "ftth")]
    Ftth,
    #[serde(rename = "ethernet")]
    Ethernet,
    #[serde(rename = "xdsl")]
    Xdsl,
    #[serde(rename = "backup_4g")]
    Backup4G,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnectionConfigurationSigAlg {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "direct_media")]
    DirectMedia,
    #[serde(rename = "any_media")]
    AnyMedia,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionConfiguration {
    pub ping: bool,
    pub is_secure_pass: bool,
    pub remote_access: bool,
    pub remote_access_port: i32,
    pub remote_access_min_port: i32,
    pub remote_access_max_port: i32,
    pub remote_access_ip: String,
    pub api_remote_access: bool,
    pub wol: bool,
    pub adblock: bool,
    pub adblock_not_set: bool,
    pub allow_token_request: bool,
    pub sip_alg: ConnectionConfigurationSigAlg,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionConfigurationBody {
    pub ping: Option<bool>,
    pub remote_access: Option<bool>,
    pub remote_access_port: Option<i32>,
    pub wol: Option<bool>,
    pub adblock: Option<bool>,
    pub allow_token_request: Option<bool>,
    pub sip_alg: Option<ConnectionConfigurationSigAlg>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FTTHStatus {
    pub sfp_present: bool,
    pub sfp_alim_ok: bool,
    pub sfp_has_power_report: bool,
    pub sfp_has_signal: bool,
    pub link: bool,
    pub sfp_serial: String,
    pub sfp_model: String,
    pub sfp_vendor: String,
    pub sfp_pwr_tx: i32, //dBm
    pub sfp_pwr_rx: i32, //dBm
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DDNSConfig {
    pub enabled: bool,
    pub hostname: String,
    pub user: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DDNSConfigBody {
    pub enabled: Option<bool>,
    pub hostname: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DDNSStatus {
    pub status: DDNSStatusKind,
    pub next_refresh: i32,
    pub last_refresh: i32,
    pub next_retry: i32,
    pub last_error: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DDNSStatusKind {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "reqfail")]
    RequestFailed,
    #[serde(rename = "authfail")]
    AuthFailed,
    #[serde(rename = "nocredential")]
    InvalidCredential,
    #[serde(rename = "ipinval")]
    InvalidIp,
    #[serde(rename = "hostinval")]
    InvalidHostname,
    #[serde(rename = "abuse")]
    Abuse,
    #[serde(rename = "dnserror")]
    DNSError,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "nowan")]
    NoWAN,
    #[serde(rename = "unknown")]
    Unknown,
}

pub type GetConnectionStatus = ResponseResult<ConnectionStatus>;
pub type GetConnectionConfiguration = ResponseResult<ConnectionConfiguration>;
pub type GetFTTHStatus = ResponseResult<FTTHStatus>;
pub type GetDDNSConfig = ResponseResult<DDNSConfig>;
pub type UpdateDDNSConfig = ResponseResult<DDNSConfig>;
pub type GetDDNSStatus = ResponseResult<DDNSStatus>;
*/