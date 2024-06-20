use std::fmt;
use std::fmt::{Display, Formatter};
use clap::ValueEnum;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use crate::app::{ResponseResult, SuccessResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::terminal::{CliDisplay, CliDisplayArg};

#[derive(Debug, Clone, Deserialize)]
pub enum LanConfigError {
    #[serde(rename = "noent")]
    InvalidId,
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "ioerror")]
    IOError,
    #[serde(rename = "inval")]
    InvalidParameter,
    #[serde(rename = "inval_gateway_ip")]
    InvalidGatewayIp,
}

#[derive(Debug, Clone, Deserialize)]
pub enum LanError {
    #[serde(rename = "inval")]
    InvalidParameter,
    #[serde(rename = "nodev")]
    InvalidInterface,
    #[serde(rename = "nohost")]
    InvalidHostId,
    #[serde(rename = "nomem")]
    InternalError,
    #[serde(rename = "netdown")]
    NetworkDown,
}

// Config

#[derive(Debug, Clone, Deserialize)]
pub struct LanConfig {
    pub ip: String,
    pub name: String,
    pub name_dns: String,
    pub name_mdns: String,
    pub name_netbios: String,
    pub mode: LanConfigType,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanConfigBody {
    pub ip: Option<String>,
    pub name: Option<String>,
    pub name_dns: Option<String>,
    pub name_mdns: Option<String>,
    pub name_netbios: Option<String>,
    pub mode: Option<LanConfigType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LanConfigType {
    #[serde(rename = "router")]
    Router,
    #[serde(rename = "bridge")]
    Bridge,
}

// Browser
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanCount {
    pub name: String,
    pub host_count: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanHost {
    pub id: String,
    pub primary_name: String,
    pub l2ident: LanHostL2Ident,
    pub host_type: LanHostType,
    pub primary_name_manual: bool,
    pub vendor_name: String,
    pub persistent: bool,
    pub reachable: bool,
    pub last_time_reachable: i32,
    pub active: bool,
    pub last_activity: i32,
    pub first_activity: i32,
    pub names: Option<Vec<LanHostName>>,
    pub l3connectivities: Vec<LanHostL3Connectivity>,
    pub network_control: Option<LanHostNetworkControl>,
}

impl CliDisplay for LanHost {
    fn json(&self) -> Value {
        json!(self)
    }

    fn stdout(&self, _arg: crate::terminal::CliDisplayArg) -> Box<dyn Display> {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Nom du device").add_attribute(Attribute::Bold),
                Cell::new("Type"),
                Cell::new("Status"),
            ]);

        table.add_row(vec![
            Cell::new(format!("{}\n({})", self.primary_name, self.vendor_name)),
            Cell::new(self.host_type.to_string()),
            Cell::new(if self.active { "Active" } else { "Inactive" })
        ]);

        Box::new(table)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanHostName {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanHostL2Ident {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LanHostL2IdentType {
    #[serde(rename = "dhcp")]
    Dhcp,
    #[serde(rename = "netbios")]
    Netbios,
    #[serde(rename = "mdns")]
    MDNSHostname,
    #[serde(rename = "mdns_srv")]
    MDNSService,
    #[serde(rename = "upnp")]
    UPnP,
    #[serde(rename = "wsd")]
    WSDiscovery,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateLanHostBody {
    pub primary_name: Option<String>,
    pub host_type: Option<LanHostType>,
    pub persistent: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanHostL3Connectivity {
    pub addr: String,
    pub af: LanHostL3ConnectivityAf,
    pub active: bool,
    pub reachable: bool,
    pub last_activity: i32,
    pub last_time_reachable: i32,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LanHostL3ConnectivityAf {
    #[serde(rename = "ipv4")]
    IPv4,
    #[serde(rename = "ipv6")]
    IPv6,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LanHostNetworkControl {
    pub profile_id: i32,
    pub name: String,
    pub current_mode: String,
}

#[derive(ValueEnum, Debug, Clone, Deserialize, Serialize)]
pub enum LanHostType {
    #[serde(rename = "workstation")]
    Workstation,
    #[serde(rename = "laptop")]
    Laptop,
    #[serde(rename = "smartphone")]
    Smartphone,
    #[serde(rename = "tablet")]
    Tablet,
    #[serde(rename = "printer")]
    Printer,
    #[serde(rename = "vg_console")]
    Console,
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "nas")]
    Nas,
    #[serde(rename = "ip_camera")]
    CameraIp,
    #[serde(rename = "ip_phone")]
    PhoneIp,
    #[serde(rename = "freebox_player")]
    FreeboxPlayer,
    #[serde(rename = "freebox_hd")]
    FreeboxHD,
    #[serde(rename = "freebox_crystal")]
    FreeboxCrystal,
    #[serde(rename = "freebox_mini")]
    FreeboxMini,
    #[serde(rename = "freebox_delta")]
    FreeboxDelta,
    #[serde(rename = "freebox_wifi")]
    FreeboxWifi,
    #[serde(rename = "freebox_one")]
    FreeboxOne,
    #[serde(rename = "freebox_pop")]
    FreeboxWifiPop,
    #[serde(rename = "networking_device")]
    NetworkDevice,
    #[serde(rename = "multimedia_device")]
    MultimediaDevice,
    #[serde(rename = "car")]
    Car,
    #[serde(rename = "other")]
    Other,
}

impl Display for LanHostType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WakeOnLanBody {
    pub mac: String,
    pub password: String,
}

impl WakeOnLanBody {
    pub fn new(mac: String, password: Option<String>) -> Self {
        Self {
            mac,
            password: password.unwrap_or_default(),
        }
    }
}

pub type ListLanCountResponse = ResponseResult<Vec<LanCount>>;
pub type ListLanResponse = ResponseResult<Vec<LanHost>>;

impl ListLanResponse {
    pub fn display(self) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Nom du device").add_attribute(Attribute::Bold),
                Cell::new("Type"),
                Cell::new("Status"),
                Cell::new("Id")
            ]);

        for x in self.result.unwrap_or_default() {
            table.add_row(vec![
                Cell::new(format!("{}\n({})", x.primary_name, x.vendor_name)),
                Cell::new(x.host_type.to_string()),
                Cell::new(if x.active {"Active" } else { "Inactive" }),
                Cell::new(x.id)
            ]);
        }

        println!("{table}");
    }
}

pub type LanHostResponse = ResponseResult<LanHost>;

impl CliDisplay for LanHostResponse {
    fn json(&self) -> Value {
        json!(self.result)
    }

    fn stdout(&self, arg: CliDisplayArg) -> Box<dyn Display> {
        self.result.as_ref().unwrap().stdout(arg)
    }
}

pub type GetLanResponse = LanHostResponse;
pub type UpdateLanResponse = LanHostResponse;
pub type WakeOnLanResponse = SuccessResponse;

pub type GetLanConfig = ResponseResult<LanConfig>;
pub type UpdateLanConfig = ResponseResult<LanConfig>;
