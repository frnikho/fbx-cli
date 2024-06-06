use crate::app::{EmptyResponse, ResponseResult, SuccessResponse};
use serde::Deserialize;

pub type SystemInfoResult = ResponseResult<SystemInfo>;

#[derive(Clone, Debug, Deserialize)]
pub struct SystemInfo {
    pub mac: String,
    pub sensors: Option<Vec<SystemSensor>>,
    pub model_info: SystemModel,
    pub fans: Option<Vec<SystemSensor>>,
    pub expansions: Option<Vec<SystemExpansion>>,
    pub box_authenticated: bool,
    pub disk_status: DiskStatus,
    pub uptime: String,
    pub uptime_val: i32,
    pub user_main_storage: String,
    pub board_name: String,
    pub serial: String,
    pub firmware_version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemSensor {
    pub id: String,
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemModel {
    pub pretty_name: String,
    pub name: SystemModelType,
    pub has_expansions: Option<bool>,
    pub has_lan_sfp: Option<bool>,
    pub has_dect: Option<bool>,
    pub has_home_automation: Option<bool>,
    pub has_femtocell_exp: Option<bool>,
    pub has_fixed_femtocell: Option<bool>,
    pub has_vm: Option<bool>,
    pub has_dsl: Option<bool>,
    pub has_standby: Option<bool>,
    pub has_eco_wifi: Option<bool>,
    pub has_wop: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SystemModelType {
    #[serde(rename = "fbxgw-r1")]
    FreeboxServerV6R1,
    #[serde(rename = "fbxgw-r2")]
    FreeboxServerV6R2,
    #[serde(rename = "fbxgw-r1/mini")]
    FreeboxMiniR1,
    #[serde(rename = "fbxgw-r2/mini")]
    FreeboxMiniR2,
    #[serde(rename = "fbxgw-r1/one")]
    FreeboxOneR1,
    #[serde(rename = "fbxgw-r2/one")]
    FreeboxOneR2,
    #[serde(rename = "fbxgw7-r1")]
    FreeboxV7R1,
    #[serde(rename = "fbxgw8-r1")]
    FreeboxV8R1,
    #[serde(rename = "fbxgw9-r1")]
    FreeboxV9R1,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemExpansion {
    #[serde(rename = "type")]
    pub kind: String,
    pub present: bool,
    pub slot: i32,
    pub probe_done: bool,
    pub supported: bool,
    pub bundle: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum DiskStatus {
    #[serde(rename = "not_detected")]
    NotDetected,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "active")]
    Active,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemUpdateStatus {
    pub state: SystemUpdateStatusState,
    pub upgrade_state: Option<UpgradeState>
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpgradeState {
    pub state: UpgradeStateKind,
    pub old_version: String,
    pub new_version: String,
    pub percent: i32,
    pub error_string: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum UpgradeStateKind {
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "download_failed")]
    DownloadFailed,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "check_failed")]
    CheckFailed,
    #[serde(rename = "prepare_write")]
    PrepareWrite,
    #[serde(rename = "prepare_write_failed")]
    PrepareWriteFailed,
    #[serde(rename = "writing")]
    Writing,
    #[serde(rename = "write_failed")]
    WriteFailed,
    #[serde(rename = "reread")]
    Reread,
    #[serde(rename = "reread_failed")]
    RereadFailed,
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "commit_failed")]
    CommitFailed,
}

#[derive(Clone, Debug, Deserialize)]
pub enum SystemUpdateStatusState {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "upgrading")]
    Upgrading,
    #[serde(rename = "up_to_date")]
    UpToDate,
    #[serde(rename = "error")]
    Error,
}

pub type GetSystemInfoRequest = ResponseResult<SystemInfo>;
pub type RebootRequest = SuccessResponse;
pub type ShutdownRequest = EmptyResponse;

pub type GetUpdateStatusResponse = ResponseResult<SystemUpdateStatus>;