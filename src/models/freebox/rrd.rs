use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RDDDatabase {
    #[serde(rename = "net")]
    Network,
    #[serde(rename = "temp")]
    Temperature,
    #[serde(rename = "dsl")]
    Dsl,
    #[serde(rename = "switch")]
    Switch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DBNetFields {
    #[serde(rename = "bw_up")]
    BwUp,
    #[serde(rename = "bw_down")]
    BwDown,
    #[serde(rename = "rate_up")]
    RateUp,
    #[serde(rename = "rate_down")]
    RateDown,
    #[serde(rename = "vpn_rate_up")]
    VpnRateUp,
    #[serde(rename = "vpn_rate_down")]
    VpnRateDown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DBTempFields {
    #[serde(rename = "cpum")]
    Cpum,
    #[serde(rename = "cpub")]
    Cpub,
    #[serde(rename = "sw")]
    Sw,
    #[serde(rename = "hdd")]
    Hdd,
    #[serde(rename = "fan_speed")]
    FanSpeed,
    #[serde(rename = "temp1")]
    Temp1,
    #[serde(rename = "temp2")]
    Temp2,
    #[serde(rename = "temp3")]
    Temp3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DBDslFields {
    #[serde(rename = "rate_up")]
    RateUp,
    #[serde(rename = "rate_down")]
    RateDown,
    #[serde(rename = "snr_up")]
    SignalNoiseRatioUp,
    #[serde(rename = "snr_down")]
    SignalNoiseRatioDown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DBSwitchFields {
    #[serde(rename = "rx_1")]
    Rx1,
    #[serde(rename = "tx_1")]
    Tx1,
    #[serde(rename = "rx_2")]
    Rx2,
    #[serde(rename = "tx_2")]
    Tx2,
    #[serde(rename = "rx_3")]
    Rx3,
    #[serde(rename = "tx_3")]
    Tx3,
    #[serde(rename = "rx_4")]
    Rx4,
    #[serde(rename = "tx_4")]
    Tx4,
}

#[derive(Clone, Debug, Serialize)]
pub struct RRDFetch {
    pub db: RDDDatabase,
    pub date_start: Option<i32>,
    pub date_end: Option<i32>,
    pub precision: Option<i32>,
    pub fields: Option<Vec<String>>,
}

pub trait RRDCalls<T: HttpClient> {
    fn fetch_rrd(&self, client: &T, body: RRDFetch) -> ResponseResult<Value>;
}