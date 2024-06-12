/*use serde::Deserialize;
use serde_json::Value;
use crate::app::ResponseResult;

#[derive(Clone, Debug, Deserialize)]
pub enum SwitchError {
    #[serde(rename = "bad_port")]
    BadPort,
    #[serde(rename = "bad_speed")]
    BadSpeed,
    #[serde(rename = "bad_link")]
    BadLink,
    #[serde(rename = "bad_mac_entry_type")]
    BadMacEntryType,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SwitchPortStatus {
    pub id: i32,
    pub link: SwitchPortStatusLink,
    pub duplex: SwitchPortStatusDuplex,
    pub speed: SwitchPortStatusSpeed,
    pub mode: String,
    pub mac_list: Vec<Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SwitchPortStatusLink {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SwitchPortStatusDuplex {
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "full")]
    Full,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SwitchPortStatusSpeed {
    #[serde(rename = "10")]
    BaseT10,
    #[serde(rename = "100")]
    BaseTX100,
    #[serde(rename = "1000")]
    BaseT1000
}

#[derive(Clone, Debug, Deserialize)]
pub struct SwitchPortConfig {
    pub id: i32,
    pub duplex: SwitchPortConfigDuplex,
    pub speed: SwitchPortConfigSpeed,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SwitchPortConfigDuplex {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "full")]
    Full,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SwitchPortConfigSpeed {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "10")]
    BaseT10,
    #[serde(rename = "100")]
    BaseTX100,
    #[serde(rename = "1000")]
    BaseT1000,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SwitchPortStats {
    pub rx_bad_bytes: i32,
    pub rx_broadcast_packets: i32,
    pub rx_bytes_rate: i32,
    pub rx_err_packets: i32,
    pub rx_fcs_packets: i32,
    pub rx_fragments_packets: i32,
    pub rx_good_bytes: i32,
    pub rx_good_packets: i32,
    pub rx_jabber_packets: i32,
    pub rx_multicast_packets: i32,
    pub rx_oversize_packets: i32,
    pub rx_packets_rate: i32,
    pub rx_pause: i32,
    pub rx_undersize_packets: i32,
    pub rx_unicast_packets: i32,
    pub tx_broadcast_packets: i32,
    pub tx_bytes: i32,
    pub tx_bytes_rate: i32,
    pub tx_collisions: i32,
    pub tx_deferred: i32,
    pub tx_excessive: i32,
    pub tx_fcs: i32,
    pub tx_late: i32,
    pub tx_multicast_packets: i32,
    pub tx_multiple: i32,
    pub tx_packets: i32,
    pub tx_packets_rate: i32,
    pub tx_pause: i32,
    pub tx_single: i32,
    pub tx_unicast_packets: i32
}

pub type ListSwitchStatus = ResponseResult<Vec<SwitchPortStatus>>;
pub type GetSwitchConfig = ResponseResult<SwitchPortConfig>;
pub type UpdateSwitchConfig = ResponseResult<SwitchPortConfig>;
pub type GetSwitchPortStats = ResponseResult<SwitchPortStats>;*/