use std::fmt::{Display, Formatter};
use comfy_table::Table;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::app::ResponseResult;
use crate::client::HttpClient;
use crate::models::display::TableDisplay;
use crate::models::freebox::wifi::access_point::WifiApBody;
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WifiBssStatusState {
    #[serde(rename = "phy_stopped")]
    ApStopped,
    #[serde(rename = "no_param")]
    NoParameters,
    #[serde(rename = "bad_param")]
    BadParameters,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WifiBssConfigEncryption {
    #[serde(rename = "wep")]
    Wep,
    #[serde(rename = "wpa_psk_auto")]
    WpaPskAuto,
    #[serde(rename = "wpa_psk_tkip")]
    WpaPskTkip,
    #[serde(rename = "wpa_psk_ccmp")]
    WpaPskCcmp,
    #[serde(rename = "wpa12_psk_auto")]
    Wpa12PskAuto,
    #[serde(rename = "wpa2_psk_auto")]
    Wpa2PskAuto,
    #[serde(rename = "wpa2_psk_tkip")]
    Wpa2PskTkip,
    #[serde(rename = "wpa2_psk_ccmp")]
    Wap2PskCcmp,
    #[serde(rename = "wpa23_psk_ccmp")]
    Wpa23PskCCmp,
}

impl Display for WifiBssConfigEncryption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WifiBssConfigEncryption::Wep => write!(f, "WEP"),
            WifiBssConfigEncryption::WpaPskAuto => write!(f, "WPA PSK Auto"),
            WifiBssConfigEncryption::WpaPskTkip => write!(f, "WPA PSK TKIP"),
            WifiBssConfigEncryption::WpaPskCcmp => write!(f, "WPA PSK CCMP"),
            WifiBssConfigEncryption::Wpa12PskAuto => write!(f, "WPA12 PSK Auto"),
            WifiBssConfigEncryption::Wpa2PskAuto => write!(f, "WPA2 PSK Auto"),
            WifiBssConfigEncryption::Wpa2PskTkip => write!(f, "WPA2 PSK TKIP"),
            WifiBssConfigEncryption::Wap2PskCcmp => write!(f, "WAP2 PSK CCMP"),
            WifiBssConfigEncryption::Wpa23PskCCmp => write!(f, "WPA23 PSK CCMP"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiBss {
    pub id: String,
    pub phy_id: i32,
    pub status: WifiBssStatus,
    pub use_shared_params: bool,
    pub config: WifiBssConfig,
    pub bss_params: WifiBssParams,
    pub shared_bss_params: WifiSharedBssParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiBssBody {
    pub use_shared_params: Option<bool>,
    pub config: Option<WifiBssConfigBody>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiBssStatus {
    pub state: WifiBssStatusState,
    pub sta_count: i32,
    pub authorized_sta_count: i32,
    pub custom_key_ssid: String,
    pub is_main_bss: bool,
    pub band: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiBssConfig {
    pub enabled: bool,
    pub wps_enabled: Option<bool>,
    pub encryption: WifiBssConfigEncryption,
    pub hide_ssid: bool,
    pub eapol_version: Option<i32>,
    pub key: String,
    pub wps_uuid: Option<String>,
    pub use_default_config: bool,
    pub ssid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiBssParams {
    pub enabled: bool,
    pub wps_uuid: Option<String>,
    pub ssid: String,
    pub encryption: WifiBssConfigEncryption,
    pub wps_enabled: bool,
    pub hide_ssid: bool,
    pub eapol_version: Option<i32>,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiSharedBssParams {
    pub enabled: bool,
    pub wps_uuid: Option<String>,
    pub ssid: String,
    pub encryption: WifiBssConfigEncryption,
    pub wps_enabled: bool,
    pub hide_ssid: bool,
    pub eapol_version: Option<i32>,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiBssConfigBody {
    pub enabled: Option<bool>,
    pub ssid: Option<String>,
    pub hide_ssid: Option<String>,
    pub encryption: Option<WifiBssConfigEncryption>,
    pub key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWifiBss(pub ResponseResult<WifiBss>);
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListWifiBss(pub ResponseResult<Vec<WifiBss>>);

impl CliDisplay for ListWifiBss {
    fn json(&self) -> Value {
        json!(self.0.result)
    }

    fn stdout(&self, _arg: CliDisplayArg) -> CliResult {
        match &self.0.result {
            Some(result) => {
                let mut table = Table::init();
                table.set_headers(vec![
                    "Ssid",
                    "Encryption",
                    "Actif",
                    "MAC"
                ]);
                result.iter().for_each(|bss| {
                    table.rows(vec![format!("{} ({})", bss.config.ssid, bss.status.band), bss.config.encryption.to_string(), bss.config.enabled.to_string(), bss.id.to_string()]);
                });
                CliResult::success(Box::new(table))
            },
            None => CliResult::error(Box::new("No BSS found."))
        }
    }

    fn raw(&self, _arg: CliDisplayArg) -> CliResult {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateWifiBss(pub ResponseResult<WifiBss>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetWifiBssDefaultConfigResponse(pub ResponseResult<WifiBssConfig>);

pub trait WifiBssCalls<T: HttpClient> {
    async fn list_wifi_bss(&self, client: &T, session: &str) -> Result<ListWifiBss, T::Error>;
    async fn get_wifi_bss(&self, client: &T, session: &str, bss_mac: &str) -> Result<GetWifiBss, T::Error>;
    async fn update_wifi_bss(&self, client: &T, session: &str, bss_mac: &str, body: WifiApBody) -> Result<GetWifiBss, T::Error>;
}