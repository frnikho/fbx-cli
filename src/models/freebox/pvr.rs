use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Debug, Clone, Error, Deserialize)]
pub enum PVRError {
    #[error("wrong id")]
    #[serde(rename = "noent")]
    NoEntity,
    #[error("invalid params")]
    #[serde(rename = "inval")]
    Invalid,
    #[error("invalid date format")]
    #[serde(rename = "inval_date_fmt")]
    InvalidDateFormat,
    #[error("start time must be before end time")]
    #[serde(rename = "invalid_end_before_start")]
    InvalidEndBeforeStart,
    #[error("system time not available")]
    #[serde(rename = "system_time_incorrect")]
    SystemTimeIncorrect,
    #[error("record duration is too long")]
    #[serde(rename = "record_duration_too_long")]
    RecordDurationTooLong,
    #[error("record date is already passed")]
    #[serde(rename = "record_date_in_past")]
    RecordDateInPast,
    #[error("unknown channel")]
    #[serde(rename = "unknown_channel")]
    UnknownChannel,
    #[error("no service for this channel")]
    #[serde(rename = "no_channel_svc")]
    NoChannelSvc,
    #[error("can’t disable manual precord")]
    #[serde(rename = "only_auto_disable")]
    OnlyAutoDisable,
    #[error("can’t change enabled state")]
    #[serde(rename = "cannot_change_en_state")]
    CannotChangeEnableState,
    #[error("can’t disable started record")]
    #[serde(rename = "cannot_disable_has_data")]
    CannotDisabledStartedRecord,
    #[error("internal error")]
    #[serde(rename = "internal_error")]
    InternalError
}

#[derive(Clone, Debug, Deserialize)]
pub enum PVRRecordState {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "start_error")]
    StartError,
    #[serde(rename = "waiting_start_time")]
    WaitingStartTime,
    #[serde(rename = "sarting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "running_error")]
    RunningError,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "finished")]
    Finished
}

#[derive(Clone, Debug, Deserialize)]
pub enum PVRRecordError {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "file_access_error")]
    FileAccess,
    #[serde(rename = "disk_full")]
    DiskFull,
    #[serde(rename = "private_but_not_private_dir")]
    PrivateButNotPrivateDir,
    #[serde(rename = "network_problem")]
    NetworkProblem,
    #[serde(rename = "resource_problem")]
    ResourceProblem,
    #[serde(rename = "no_stream_available")]
    NoStreamAvailable,
    #[serde(rename = "no_data_received")]
    NoDataReceived,
    #[serde(rename = "missed")]
    Missed,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "internal_error")]
    Internal,
    #[serde(rename = "unknown_error")]
    Unknown,
}

#[derive(Clone, Debug, Deserialize)]
pub enum PvrRecordChannelQuality {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "hd")]
    Hd,
    #[serde(rename = "sd")]
    Sd,
    #[serde(rename = "ld")]
    Ld,
    #[serde(rename = "3d")]
    D3
}

#[derive(Clone, Debug, Deserialize)]
pub enum PvrRecordChannelType {
    #[serde(rename = "")]
    Auto,
    #[serde(rename = "iptv")]
    Iptv,
    #[serde(rename = "dvd")]
    Dvd,
}

#[derive(Clone, Debug, Deserialize)]
pub enum PvrRecordBroadcastType {
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "radio")]
    Radio
}

#[derive(Clone, Debug, Deserialize)]
pub struct PvrConfig {
    pub margin_before: i32,
    pub margin_after: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct PvrConfigBody {
    pub margin_before: Option<i32>,
    pub margin_after: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PvrQuota {
    pub quota_exceeded: bool,
    pub needed_tresh: i32,
    pub cur_tresh: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PvrRecord {
    pub id: String,
    pub media: Option<String>,
    pub path: String,
    pub has_record_gen: bool,
    pub record_gen_id: i32,
    pub conflict: bool,
    pub overlap_list: Vec<i32>,
    pub enabled: bool,
    pub altered: bool,
    pub state: PVRRecordState,
    pub error: PVRRecordError,
    pub channel_uuid: String,
    pub channel_name: String,
    pub channel_quality: String,
    pub channel_type: String,
    pub name: String,
    pub subname: String,
    pub broadcast_type: String,
    pub start: i32,
    pub end: i32,
    pub secure: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct PvrRecordBody {
    pub name: Option<String>,
    pub subname: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Media {
    pub media: String,
    pub free_bytes: i32,
    pub total_bytes: i32,
    pub record_time: i32,
}

pub struct PvrConfigResponse(ResponseResult<PvrConfig>);
pub struct PvrQuotaResponse(ResponseResult<PvrQuota>);
pub struct PvrRecordsResponse(ResponseResult<Vec<PvrRecord>>);
pub struct PvrRecordResponse(ResponseResult<PvrRecord>);

pub struct StorageMediaResponse(ResponseResult<Vec<Media>>);

pub trait PvrCalls<T: HttpClient> {
    async fn get_pvr_config(&self, session: &str) -> Result<PvrConfigResponse, PVRError>;
    async fn update_pvr_config(&self, session: &str, body: PvrConfigBody) -> Result<PvrConfigResponse, PVRError>;
    async fn get_pvr_quota(&self, session: &str) -> Result<PvrQuotaResponse, PVRError>;
    async fn next_pvr_quota(&self, session: &str) -> Result<PvrQuotaResponse, PVRError>;
    async fn get_pvr_record(&self, session: &str, id: &str) -> Result<PvrRecordResponse, PVRError>;
    async fn get_pvr_records(&self, session: &str) -> Result<PvrRecordsResponse, PVRError>;
    async fn update_pvr_record(&self, session: &str, id: &str, body: PvrRecordBody) -> Result<PvrRecordResponse, PVRError>;
    async fn delete_pvr_record(&self, session: &str, id: &str) -> Result<PvrRecordResponse, PVRError>;
    async fn list_storage_media(&self, session: &str) -> Result<StorageMediaResponse, PVRError>;
}