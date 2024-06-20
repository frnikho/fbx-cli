use std::fmt::Display;
use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::terminal::{CliDisplay, CliDisplayArg};

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub struct LcdUpdateBody {
    pub brightness: Option<u8>,
    pub orientation: Option<u16>,
    pub hide_wifi_key: Option<bool>,
    pub orientation_forced: Option<bool>,
}

impl LcdUpdateBody {
    pub fn brightness(brightness: u8) -> Self {
        Self {
            brightness: Some(brightness),
            orientation: None,
            hide_wifi_key: None,
            orientation_forced: None,
        }
    }

    pub fn orientation(orientation: u16) -> Self {
        Self {
            brightness: None,
            orientation: Some(orientation),
            hide_wifi_key: None,
            orientation_forced: Some(true),
        }
    }

    pub fn hide_wifi_key(hide: bool) -> Self {
        Self {
            brightness: None,
            orientation: None,
            hide_wifi_key: Some(hide),
            orientation_forced: None
        }
    }
}

pub type LcdInfoResponse = ResponseResult<LcdInfoResult>;

impl CliDisplay for LcdInfoResponse {
    fn json(&self) -> Value {
        json!(self.result)
    }

    fn stdout(&self, _arg: CliDisplayArg) -> Box<dyn Display> {
        match &self.result {
            Some(res) => {
                Box::new(format!(
                    "Brightness: {}\nOrientation: {}\nHide Wifi Key: {}",
                    res.brightness, res.orientation, res.hide_wifi_key
                ))
            }
            None => Box::new("Error"),
        }
    }
}

pub type LcdUpdateResponse = ResponseResult<LcdInfoResult>;
