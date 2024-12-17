use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PortForwardingConfig {
    pub id: i32,
    pub enabled: bool,
    pub ip_proto: IpProtocol,
    pub wan_port_start: String,
    pub wan_port_end: i32,
    pub lan_ip: String,
    pub lan_port: i32,
    pub hostname: String,
    //TODO: pub host: LanHost,
    pub src_ip: String,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncomingPortConfig {

}

#[derive(Debug, Clone, Deserialize)]
pub enum IncomingPortConfigId {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "bittorrent-main")]
    BittorrentMain,
    BittorrentDht,
    OpenVpnRouted,
    OpenVpnBridge,
    IpsecIke,
    IpsecNat,
    Pptp,
    Ftp,
    FtpPasv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardingConfigUpdateBody {
    pub id: Option<i32>,
    pub enabled: Option<bool>,
    pub ip_proto: Option<IpProtocol>,
    pub wan_port_start: Option<String>,
    pub wan_port_end: Option<i32>,
    pub lan_ip: Option<String>,
    pub lan_port: Option<i32>,
    pub src_ip: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForwardingConfigCreateBody {
    pub enabled: bool,
    pub ip_proto: IpProtocol,
    pub wan_port_start: String,
    pub wan_port_end: i32,
    pub lan_ip: String,
    pub lan_port: i32,
    pub src_ip: String,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpProtocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

pub type ListPortForwardingResponse = ResponseResult<Vec<PortForwardingConfig>>;
pub type GetPortForwardingResponse = ResponseResult<PortForwardingConfig>;
pub type UpdatePortForwardingResponse = ResponseResult<PortForwardingConfig>;
