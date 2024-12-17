use crate::app::{EmptyResponse, ResponseResult, SuccessResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::terminal::{CliDisplay, CliDisplayArg, CliResult};

pub type SystemInfoResult = ResponseResult<SystemInfo>;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemSensor {
    pub id: String,
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemExpansion {
    #[serde(rename = "type")]
    pub kind: String,
    pub present: bool,
    pub slot: i32,
    pub probe_done: bool,
    pub supported: bool,
    pub bundle: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemUpdateStatus {
    pub state: SystemUpdateStatusState,
    pub upgrade_state: Option<UpgradeState>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpgradeState {
    pub state: UpgradeStateKind,
    pub old_version: String,
    pub new_version: String,
    pub percent: i32,
    pub error_string: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl CliDisplay for GetSystemInfoRequest {
    fn json(&self) -> Value {
        json!(self.clone().result)
    }

    fn stdout(&self, _: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new("System info"))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

pub type RebootRequest = SuccessResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShutdownRequest(EmptyResponse);

impl CliDisplay for ShutdownRequest {
    fn json(&self) -> Value {
        json!(r#"{"success": true}"#)
    }

    fn stdout(&self, _: CliDisplayArg) -> CliResult {
        CliResult::success(Box::new("Extinction en cours..."))
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}

pub type GetUpdateStatusResponse = ResponseResult<SystemUpdateStatus>;

impl CliDisplay for GetUpdateStatusResponse {
    fn json(&self) -> Value {
        json!(self.result)
    }

    fn stdout(&self, _: CliDisplayArg) -> CliResult {
        match &self.result {
            Some(result) => CliResult::success(Box::new(match result.state {
                SystemUpdateStatusState::UpToDate => "Système à jour ✔",
                SystemUpdateStatusState::Error => "Erreur pendant la mise à jour ❌",
                SystemUpdateStatusState::Initializing => "Initialisation de la mise à jour",
                SystemUpdateStatusState::Upgrading => "Mise à jour en cours",
            })),
            None => CliResult::error(Box::new("Pas de données disponibles pour le moment"))
        }
    }

    fn raw(&self, _: CliDisplayArg) -> CliResult {
        todo!()
    }
}