use crate::app::ResponseResult;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LcdInfoResult {
    pub brightness: i32,
    pub orientation_forced: bool,
    pub orientation: i32,
    pub hide_wifi_key: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub enum LcdError {
    #[serde(rename = "inval")]
    Invalid,
    #[serde(rename = "no_panel")]
    NoScreen,
    #[serde(rename = "setup")]
    Setup,
    #[serde(rename = "notsup")]
    NotSupported,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LcdUpdateBody {
    pub brightness: Option<i32>,
    pub orientation: Option<i32>,
    pub hide_wifi_key: Option<bool>,
}

pub type LcdInfoResponse = ResponseResult<LcdInfoResult>;
pub type LcdUpdateResponse = ResponseResult<LcdInfoResult>;
