use serde::{Deserialize, Serialize};
use crate::app::{ResponseResult, SuccessResponse};
use crate::client::HttpClient;
use crate::models::freebox::configuration::lan::LanHost;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WifiCustomKeyParamsAccessType {
    Full,
    NetOnly,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiCustomKeyHost {
    pub hostname: String,
    pub host: Option<LanHost>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiCustomKeyParams {
    pub description: String,
    pub key: String,
    pub max_use_count: i32,
    pub duration: i32,
    pub access_type: WifiCustomKeyParamsAccessType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WifiCustomKey {
    pub id: i32,
    pub remaining: i32,
    pub params: WifiCustomKeyParams,
    pub users: Vec<WifiCustomKeyHost>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WifiCustomKeyBody {
    pub description: String,
    pub key: String,
    pub max_use_count: i32,
    pub duration: i32,
    pub access_type: WifiCustomKeyParamsAccessType,
}

pub type ListWifiCustomKeys = ResponseResult<Vec<WifiCustomKey>>;
pub type GetWifiCustomKey = ResponseResult<WifiCustomKey>;
pub type DeleteWifiCustomKey = SuccessResponse;
pub type CreateWifiCustomKey = ResponseResult<WifiCustomKey>;

pub trait WifiGuestCalls<T: HttpClient> {
    async fn list_wifi_guest_access(&self, client: &T, session: &str) -> Result<ListWifiCustomKeys, T::Error>;
    async fn get_wifi_guest_access(&self, client: &T, session: &str, key: &str) -> Result<GetWifiCustomKey, T::Error>;
    async fn delete_wifi_guest_access(&self, client: &T, session: &str, key: &str) -> Result<SuccessResponse, T::Error>;
    async fn create_wifi_guest_access(&self, client: &T, session: &str, body: WifiCustomKeyBody) -> Result<GetWifiCustomKey, T::Error>;
}