use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Clone, Debug, Error, Deserialize)]
pub enum VPNClientError {
    #[error("invalid parameters")]
    #[serde(rename = "inval")]
    Invalid,
    #[error("internal error")]
    #[serde(rename = "nomem")]
    NoMemory,
    #[error("internal error")]
    #[serde(rename = "ioerror")]
    IOError,
    #[error("invalid device")]
    #[serde(rename = "nodev")]
    NoDevice,
    #[error("invalid id")]
    #[serde(rename = "noent")]
    NoEntity,
    #[error("network is not available")]
    #[serde(rename = "netdown")]
    NetworkDown,
    #[error("entry already exists")]
    #[serde(rename = "exist")]
    AlreadyExist,
    #[error("resource is busy")]
    #[serde(rename = "busy")]
    Busy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VPNClientType {
    #[serde(rename = "pptp")]
    Pptp,
    #[serde(rename = "openvpn")]
    OpenVPN,
    #[serde(rename = "wireguard")]
    WireGuard,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VPNClientConfig {
    pub id: String,
    pub description: String,
    pub r#type: VPNClientType,
    pub active: bool,
    pub conf_pptp: VPNClientConfigPPTP,
    pub conf_wireguard: VPNClientConfigWireGuard,
}

#[derive(Debug, Clone, Serialize)]
pub struct VPNClientConfigCreateBody {
    pub id: String,
    pub description: String,
    pub r#type: VPNClientType,
    pub active: bool,
    pub conf_pptp: Option<VPNClientConfigPPTP>,
    pub conf_wireguard: Option<VPNClientConfigWireGuard>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VPNClientConfigPPTP {
    pub remote_host: String,
    pub username: String,
    pub mppe: String,
    pub allowed_auth: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VPNClientConfigWireGuard {
    pub remote_addr: String,
    pub remote_port: i32,
    pub remote_public_key: String,
    pub remote_preshared_key: String,
    pub local_priv_key: String,
    pub local_addr: Vec<VPNClientConfigWireGuardIP>,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VPNClientConfigWireGuardIP {
    pub ip: String,
    pub len: i32,
}

pub struct VPNClientsConfigResponse(pub ResponseResult<Vec<VPNClientConfig>>);
pub struct VPNClientConfigResponse(pub ResponseResult<VPNClientConfig>);

pub trait VpnClientCalls<T: HttpClient> {
    async fn list_vpn_client_config(&self, session: &str) -> Result<VPNClientsConfigResponse, VPNClientError>;
    async fn get_vpn_client_config(&self, session: &str, id: &str) -> Result<VPNClientConfigResponse, VPNClientError>;
    async fn update_vpn_client_config(&self, session: &str, config: VPNClientConfig) -> Result<VPNClientConfig, VPNClientError>;
}