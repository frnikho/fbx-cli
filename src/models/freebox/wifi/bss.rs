use serde::Deserialize;
use crate::app::ResponseResult;
use crate::client::HttpClient;
use crate::models::freebox::wifi::access_point::WifiApBody;

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBss {
    pub id: i32,
    pub phy_id: String,
    pub status: WifiBssStatus,
    pub use_shared_params: bool,
    pub config: WifiBssConfig,
    pub bss_params: WifiBssConfig,
    pub shared_bss_params: WifiBssConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBssBody {
    pub use_shared_params: Option<bool>,
    pub config: Option<WifiBssConfigBody>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBssStatus {
    pub state: WifiBssStatusState,
    pub sta_count: i32,
    pub authorized_sta_count: i32,
    pub custom_key_ssid: String,
    pub is_main_bss: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBssConfig {
    pub enabled: bool,
    pub use_default_config: bool,
    pub ssid: String,
    pub hide_ssid: String,
    pub encryption: WifiBssConfigEncryption,
    pub key: String,
    pub eapol_version: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiBssConfigBody {
    pub enabled: Option<bool>,
    pub ssid: Option<String>,
    pub hide_ssid: Option<String>,
    pub encryption: Option<WifiBssConfigEncryption>,
    pub key: Option<String>,
}

pub type GetWifiBss = ResponseResult<WifiBss>;
pub type ListWifiBss = ResponseResult<Vec<WifiBss>>;
pub type UpdateWifiBss = ResponseResult<WifiBss>;
pub type GetWifiBssDefaultConfigResponse = ResponseResult<WifiBssConfig>;

pub trait WifiBssCalls<T: HttpClient> {
    async fn list_wifi_bss(&self, client: &T, session: &str) -> Result<ListWifiBss, T::Error>;
    async fn get_wifi_bss(&self, client: &T, session: &str, bss_mac: &str) -> Result<GetWifiBss, T::Error>;
    async fn update_wifi_bss(&self, client: &T, session: &str, bss_mac: &str, body: WifiApBody) -> Result<GetWifiBss, T::Error>;

}