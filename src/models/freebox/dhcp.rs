/*use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};
use crate::models::freebox::lan::LanHost;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DHCPError {
    #[serde(rename = "inval")]
    Invalid,
    #[serde(rename = "inval_netmask")]
    InvalidNetmask,
    #[serde(rename = "inval_ip_range")]
    InvalidIpRange,
    #[serde(rename = "inval_ip_range_net")]
    MismatchIpRangeNetmask,
    #[serde(rename = "inval_gw_net")]
    MismatchGatewayNetmask,
    #[serde(rename = "exist")]
    AlreadyExist,
    #[serde(rename = "nodev")]
    NoDevice,
    #[serde(rename = "noent")]
    NoEntry,
    #[serde(rename = "netdown")]
    NetworkDown,
    #[serde(rename = "busy")]
    Busy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHCPConfig {
    pub enabled: bool,
    pub sticky_assign: bool,
    pub gateway: String,
    pub netmask: String,
    pub ip_range_start: String,
    pub ip_range_end: String,
    pub always_broadcast: bool,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHCPStaticLease {
    pub id: String,
    pub mac: String,
    pub comment: String,
    pub hostname: String,
    pub ip: String,
    pub host: Option<LanHost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHCPDynamicLease {
    pub mac: String,
    pub hostname: String,
    pub ip: String,
    pub lease_remaining: i32,
    pub assign_time: i32,
    pub refresh_time: i32,
    pub is_static: bool,
    pub host: Option<LanHost>
}

pub type ListDHCPStaticLeasesResponse = ResponseResult<Vec<DHCPStaticLease>>;
pub type ListDHCPDynamicLeasesResponse = ResponseResult<Vec<DHCPDynamicLease>>;
pub type GetDHCPStaticLeasesResponse = ResponseResult<DHCPStaticLease>;
pub type GetDHCPDynamicLeasesResponse = ResponseResult<DHCPDynamicLease>;
*/