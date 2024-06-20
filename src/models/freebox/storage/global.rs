use serde::Deserialize;
use thiserror::Error;
use crate::app::ResponseResult;

#[derive(Debug, Clone, Error, Deserialize)]
pub enum StorageError {
    #[error("No disk/partition with this id")]
    #[serde(rename = "not_found")]
    NotFound,
    #[error("No such disk")]
    #[serde(rename = "invalid_disk")]
    InvalidDisk,
    #[error("This is not a disk but a partition")]
    #[serde(rename = "is_a_partition")]
    IsAPartition,
    #[error("This action is not permitted on internal disk")]
    #[serde(rename = "is_internal")]
    IsInternal,
    #[error("Operation not supported")]
    #[serde(rename = "op_not_supported")]
    OperationNotSupported,
    #[error("Operation failed")]
    #[serde(rename = "op_failed")]
    OperationFailed,
    #[error("Disk is busy")]
    #[serde(rename = "disk_busy")]
    DiskBusy,
    #[error("Parition not found")]
    #[serde(rename = "partition_not_found")]
    PartitionNotFound,
    #[error("Partition needed")]
    #[serde(rename = "partition_needed")]
    PartitionNeeded,
}

#[derive(Debug, Clone, Deserialize)]
pub enum DiskPartitionState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "formatting")]
    Formatting,
    #[serde(rename = "mounting")]
    Mounting,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "mounted")]
    Mounted,
    #[serde(rename = "unmounted")]
    Unmounting,
    #[serde(rename = "umounted")]
    Unmounted,
    #[serde(rename = "ejecting")]
    Ejecting,
}

#[derive(Debug, Clone, Deserialize)]
pub enum DiskPartitionFSType {
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "xfs")]
    Xfs,
    #[serde(rename = "ext4")]
    Ext4,
    #[serde(rename = "vfat")]
    VFat,
    #[serde(rename = "ntfs")]
    Ntfs,
    #[serde(rename = "hf")]
    Hf,
    #[serde(rename = "hfplus")]
    HfPlus,
    #[serde(rename = "swap")]
    Swap,
    #[serde(rename = "exfat")]
    ExFat,
}

#[derive(Debug, Clone, Deserialize)]
pub enum DiskPartitionFSResult {
    #[serde(rename = "no_run_yet")]
    NoRunYet,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "fs_clean")]
    FsClean,
    #[serde(rename = "fs_corrected")]
    FsCorrected,
    #[serde(rename = "fs_needs_correction")]
    FsNeedsCorrection,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Deserialize)]
pub enum StorageDiskType {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "sata")]
    Sata,
    #[serde(rename = "nvme")]
    Nvme
}

#[derive(Debug, Clone, Deserialize)]
pub enum StorageDiskState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "formatting")]
    Formatting
}

#[derive(Debug, Clone, Deserialize)]
pub enum StorageDiskTableType {
    #[serde(rename = "msdos")]
    MsDos,
    #[serde(rename = "gpt")]
    Gpt,
    #[serde(rename = "superfloppy")]
    SuperFloppy,
    #[serde(rename = "empty")]
    Empty
}

#[derive(Debug, Clone, Deserialize)]
pub struct OperationProgress {
    pub done_steps: i32,
    pub max_steps: i32,
    pub percent: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiskPartition {
    pub id: i32,
    pub disk_id: i32,
    pub state: DiskPartitionState,
    pub fstype: DiskPartitionFSType,
    pub label: String,
    pub path: String,
    pub total_bytes: i32,
    pub used_bytes: i32,
    pub free_bytes: i32,
    pub fsck_result: DiskPartitionFSResult,
    pub operation_pct: OperationProgress,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageDisk {
    pub id: i32,
    pub r#type: StorageDiskType,
    pub state: StorageDiskState,
    pub connector: i32,
    pub total_bytes: i32,
    pub table_type: i32, //see mapping enum 'StorageDiskTableType'
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub temp: i32,
    pub operation_pct: OperationProgress,
    pub partitions: Vec<DiskPartition>,
    pub idle: Option<bool>,
    pub idle_duration: Option<i32>,
    pub spinning: Option<bool>,
    pub active_duration: Option<i32>,
    pub time_before_spindown: Option<i32>,
    pub read_requests: Option<i32>,
    pub read_error_requests: Option<i32>,
    pub write_requests: Option<i32>,
    pub write_error_requests: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub external_pm_enabled: bool,
    pub external_pm_idle_before_spindown: i32,
}

pub struct ListStorageDiskResponse(pub ResponseResult<Vec<StorageDisk>>);
pub struct GetStorageDiskResponse(pub ResponseResult<StorageDisk>);
pub struct UpdateStorageDiskStateResponse(pub ResponseResult<StorageDisk>); //TODO: create struct request body