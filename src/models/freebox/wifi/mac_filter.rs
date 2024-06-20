use serde::{Deserialize, Serialize};
use crate::app::{ResponseResult, SuccessResponse};
use crate::client::HttpClient;
use crate::models::freebox::configuration::lan::LanHost;

#[derive(Debug, Clone, Deserialize)]
pub enum MacFilterState {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WifiMacFilterType {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiMacFilter {
    pub id: String,
    pub mac: String,
    pub comment: String,
    pub r#type: WifiMacFilterType,
    pub hostname: String,
    pub host: Option<LanHost>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiMacFilterBody {
    pub comment: Option<String>,
    pub r#type: Option<WifiMacFilterType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiMacFilterCreate {
    pub comment: String,
    pub r#type: WifiMacFilterType,
    pub mac: String,
}

pub type GetWifiMacFilter = ResponseResult<WifiMacFilter>;
pub type ListWifiMacFilter = ResponseResult<Vec<WifiMacFilter>>;
pub type UpdateWifiMacFilter = ResponseResult<WifiMacFilter>;
pub type DeleteWifiMacFilter = SuccessResponse;
pub type CreateWifiMacFilter = ResponseResult<WifiMacFilter>;

pub trait WifiMacFilterCalls<T: HttpClient> {
    async fn list_wifi_mac_filter(&self, client: &T, session: &str) -> Result<ListWifiMacFilter, T::Error>;
    async fn get_wifi_mac_filter(&self, client: &T, session: &str, mac: &str) -> Result<WifiMacFilter, T::Error>;
    async fn update_wifi_mac_filter(&self, client: &T, session: &str, mac: &str, body: WifiMacFilterBody) -> Result<WifiMacFilter, T::Error>;
    async fn delete_wifi_mac_filter(&self, client: &T, session: &str, mac: &str) -> Result<DeleteWifiMacFilter, T::Error>;
    async fn create_wifi_mac_filter(&self, client: &T, session: &str, body: WifiMacFilterCreate) -> Result<WifiMacFilter, T::Error>;
}