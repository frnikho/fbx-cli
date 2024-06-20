use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Error, Debug, Clone, Deserialize)]
pub enum StandbyError {
    #[error("Invalid parameters")]
    #[serde(rename = "inval")]
    Invalid
}

#[derive(Debug, Clone, Deserialize,  Serialize)]
pub enum StandbyMode {
    #[serde(rename = "wifi_off")]
    WifiOff,
    #[serde(rename = "standby")]
    Standby,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandbyConfig {
    pub use_planning: bool,
    pub planning_mode: StandbyMode,
    pub resolution: i32,
    pub mapping: Vec<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StandbyConfigUpdate {
    pub use_planning: Option<bool>,
    pub planning_mode: Option<StandbyMode>,
    pub mapping: Option<Vec<bool>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StandbyStatus {
    pub use_planning: bool,
    pub planning_mode: StandbyMode,
    pub next_change: i32,
    pub available_planning_modes: Vec<StandbyMode> 
}

pub struct GetStandbyStatusResponse(pub ResponseResult<StandbyStatus>);
pub struct GetStandbyConfigResponse(pub ResponseResult<StandbyConfig>);
pub struct UpdateStandbyConfigResponse(pub ResponseResult<StandbyConfig>);

pub trait StandbyCalls<T: HttpClient> {
    fn get_standby_status(&self, client: &T) -> GetStandbyStatusResponse;
    fn get_standby_config(&self, client: &T) -> GetStandbyConfigResponse;
    fn update_standby_config(&self, client: &T, body: StandbyConfigUpdate) -> UpdateStandbyConfigResponse;
}