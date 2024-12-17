use comfy_table::{Table};
use crate::app::ResponseResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::models::display::TableDisplay;
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LcdInfoResult {
    pub brightness: i32,
    pub orientation_forced: bool,
    pub orientation: i32,
    pub hide_wifi_key: bool,
}

impl LcdInfoResult {
    pub fn hide_wifi_key_to_str(&self) -> &str {
        match self.hide_wifi_key {
            true => "Oui",
            false => "Non",
        }
    }
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

    fn stdout(&self, _: CliDisplayArg) -> CliResult {
        match &self.result {
            Some(result) => {
                let mut table = Table::init();
                table.set_headers(vec![
                    "Luminosité",
                    "Orientation de l'écran",
                    "Clé wifi caché",
                ]);
                table.rows(vec![format!("{}%", result.brightness), format!("{}°", result.orientation), result.hide_wifi_key_to_str().to_string()]);
                CliResult::success(Box::new(table))
            },
            None => CliResult::error(Box::new("No data")),
        }
        
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        match &self.result {
            Some(result) => {
                CliResult::success(Box::new(format!("Luminosité: {}%\nOrientation de l'écran: {}°\nClé wifi caché: {}", result.brightness, result.orientation, result.hide_wifi_key_to_str())))
            },
            None => CliResult::error(Box::new("No data")),
        }
    }
}

pub type LcdUpdateResponse = ResponseResult<LcdInfoResult>;