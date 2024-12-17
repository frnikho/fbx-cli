use crate::app::{ResponseResult, SuccessResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use crate::models::args::{CreateLeaseArgs, DhcpSetArgs, UpdateLeaseArgs};
use crate::models::freebox::configuration::lan::LanHost;
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Error, Clone, Deserialize)]
pub enum DHCPv6Error {
    #[error("Invalid parameters")]
    #[serde(rename = "inval")]
    InvalidParameters,
    #[error("no such entry")]
    #[serde(rename = "noent")]
    NoEntry,
    #[error("too many entries")]
    #[serde(rename = "nospc")]
    TooManyEntries,
    #[error("already exists")]
    #[serde(rename = "exist")]
    AlreadyExists,
    #[error("conflict with another rule")]
    #[serde(rename = "conflict")]
    Conflict,
    #[error("Internal error")]
    #[serde(rename = "nomem")]
    Internal
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct DHCPConfigBody {
    pub enabled: Option<bool>,
    pub sticky_assign: Option<bool>,
    pub ip_range_start: Option<String>,
    pub ip_range_end: Option<String>,
    pub always_broadcast: Option<bool>,
    pub dns: Option<Vec<String>>,
}

impl From<DhcpSetArgs> for DHCPConfigBody {
    fn from(value: DhcpSetArgs) -> Self {
        Self {
            enabled: value.enable,
            sticky_assign: value.sticky,
            ip_range_start: value.ip_start,
            ip_range_end: value.ip_end,
            always_broadcast: value.broadcast,
            dns: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DHCPDns(pub DHCPConfig);
#[derive(Debug, Clone, Deserialize)]
pub struct DHCPv6Dns(pub DHCPv6Config);

impl CliDisplay for DHCPDns {
    fn json(&self) -> Value {
        json!(self.0.dns)
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new(self.0.dns.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

impl CliDisplay for DHCPv6Dns {
    fn json(&self) -> Value {
        json!(self.0.dns)
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new(self.0.dns.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DHCPv6Config {
    pub enabled: bool,
    pub use_custom_dns: bool,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DHCPv6ConfigBody {
    pub enabled: Option<bool>,
    pub use_custom_dns: Option<bool>,
}

impl From<DhcpSetArgs> for DHCPv6ConfigBody {
    fn from(value: DhcpSetArgs) -> Self {
        Self {
            enabled: value.enable,
            use_custom_dns: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DHCPStaticLease {
    pub id: String,
    pub mac: String,
    pub comment: String,
    pub hostname: String,
    pub ip: String,
    pub host: Option<LanHost>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DHCPStaticLeaseUpdateBody {
    pub mac: Option<String>,
    pub comment: Option<String>,
    pub ip: Option<String>,
}

impl From<UpdateLeaseArgs> for DHCPStaticLeaseUpdateBody {
    fn from(value: UpdateLeaseArgs) -> Self {
        Self {
            mac: value.mac,
            comment: value.comment,
            ip: value.ip,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DHCPStaticLeaseCreateBody {
    pub ip: String,
    pub mac: String,
    pub comment: Option<String>,
}

impl From<CreateLeaseArgs> for DHCPStaticLeaseCreateBody {
    fn from(value: CreateLeaseArgs) -> Self {
        Self {
            ip: value.ip,
            mac: value.mac,
            comment: value.comment,
        }
    }
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

pub type DHCPConfigResponse = ResponseResult<DHCPConfig>;

impl CliDisplay for DHCPConfigResponse {
    fn json(&self) -> Value {
        todo!()
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        todo!()
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

pub type DHCPv6ConfigResponse = ResponseResult<DHCPv6Config>;

impl CliDisplay for DHCPv6ConfigResponse {
    fn json(&self) -> Value {
        todo!()
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        todo!()
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

pub type ListDHCPStaticLeasesResponse = ResponseResult<Vec<DHCPStaticLease>>;
pub type ListDHCPDynamicLeasesResponse = ResponseResult<Vec<DHCPDynamicLease>>;
pub type DHCPStaticLeasesResponse = ResponseResult<DHCPStaticLease>;
pub type DHCPDynamicLeasesResponse = ResponseResult<DHCPDynamicLease>;
pub type DeleteDHCPStaticLeaseResponse = ResponseResult<SuccessResponse>;