use serde::{Deserialize};
use serde_json::Value;
use thiserror::Error;
use crate::app::ResponseResult;

#[derive(Error, Clone, Debug, Deserialize)]
pub enum VPNServerError {
    #[error("Invalid parameters")]
    #[serde(rename = "inval")]
    Invalid,
    #[error("entry alreadt exists")]
    #[serde(rename = "exist")]
    AlreadyExist,
    #[error("invalid id")]
    #[serde(rename = "noent")]
    InvalidId,
    #[error("internal error")]
    #[serde(rename = "nomem")]
    NoMemory,
    #[error("Not supported")]
    #[serde(rename = "unsupp")]
    Unsupported,
    #[error("Resource in use")]
    #[serde(rename = "inuse")]
    ResourceInUse,
    #[error("Resource is busy")]
    #[serde(rename = "busy")]
    ResourceBusy,
    #[error("internal error")]
    #[serde(rename = "ioerror")]
    Internal,
    #[error("too many elements")]
    #[serde(rename = "size")]
    Size,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNServer {
    pub name: String,
    pub r#type: VPNServerType,
    pub state: VPNServerState,
    pub connection_count: i32,
    pub auth_connection_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VPNServerType {
    #[serde(rename = "ipsec")]
    Ipsec,
    #[serde(rename = "pptp")]
    Pptp,
    #[serde(rename = "openvpn")]
    OpenVpn,
    #[serde(rename = "wireguard")]
    WireGuard,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VPNServerState {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNPPTPConfig {
    pub mppe: VPNServerConfigMmpe,
    pub allowed_auth: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VPNServerConfigMmpe {
    #[serde(rename = "disable")]
    Disable,
    #[serde(rename = "require")]
    Require,
    #[serde(rename = "require_128")]
    Require128
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNOpenVpnConfig {
    pub cipher: VPNOpenVpnConfigCipher,
    pub disable_fragment: bool,
    pub use_tcp: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNIPSecAuthMode {
    pub id_source: String,
    pub id_custom: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNIpSecConfig {
    pub ike_version: i32,
    pub auth_modes: Vec<VPNIPSecAuthMode>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VPNOpenVpnConfigCipher {
    #[serde(rename = "blownfish")]
    Blowfish,
    #[serde(rename = "aes128")]
    Aes128,
    #[serde(rename = "aes256")]
    Aes256,
    #[serde(rename = "chacha20poly1305")]
    ChaCha20,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNWireGuardConfig {
    pub mtu: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNServerConfig {
    pub id: String,
    pub r#type: VPNServerConfigType,
    pub enabled: bool,
    pub enable_ipv4: bool,
    pub enable_ipv6: bool,
    pub port: i32,
    pub min_port: i32,
    pub max_port: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VPNServerConfigType {
    Pptp,
    OpenVpn,
    IpSec,
    WireGuard,
}

pub type ListVPNServerResponse = ResponseResult<Vec<VPNServer>>;
