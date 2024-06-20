use serde::Deserialize;
use thiserror::Error;
use crate::app::ResponseResult;
use crate::client::HttpClient;

#[derive(Debug, Clone, Error, Deserialize)]
pub enum RAIDError {
    #[error("Invalid parameters(s)")]
    #[serde(rename = "inval")]
    Invalid,
    #[error("Function not available")]
    #[serde(rename = "no_sys")]
    UnavailableFunction,
    #[error("No member found")]
    #[serde(rename = "member_not_found")]
    MemberNotFound,
    #[error("Too many members")]
    #[serde(rename = "members_too_many")]
    TooManyMembers,
    #[error("RAID array not found")]
    #[serde(rename = "array_not_found")]
    ArrayNotFound,
    #[error("Error when stopping the RAID array")]
    #[serde(rename = "array_stop_failed")]
    ArrayStopFailed,
    #[error("Error when starting the RAID array")]
    #[serde(rename = "array_start_failed")]
    ArrayStartFailed,
    #[error("Error when destroying the RAID array")]
    #[serde(rename = "array_destroy_failed")]
    ArrayDestroyFailed,
    #[error("The RAID array is not active")]
    #[serde(rename = "array_not_running")]
    ArrayNotRunning,
    #[error("The RAID array is not stopped")]
    #[serde(rename = "array_not_stopped")]
    ArrayNotStopped,
    #[error("The RAID array is degraded")]
    #[serde(rename = "array_degraded")]
    ArrayDegraded,
    #[error("The RAID array is not degraded")]
    #[serde(rename = "array_not_degraded")]
    ArrayNotDegraded,
    #[error("The RAID array is full")]
    #[serde(rename = "array_complete")]
    ArrayComplete,
    #[error("The specified disks are already members of a RAID array")]
    #[serde(rename = "already_member")]
    AlreadyMember,
    #[error("The same disk has been specified more than once")]
    #[serde(rename = "disk_more_than_once")]
    DiskMoreThanOnce,
    #[error("Insufficient number of disks")]
    #[serde(rename = "disks_missing")]
    DiskMissing,
    #[error("Only internal drives can be used in a RAID array")]
    #[serde(rename = "bad_disk_location")]
    BadDiskLocation,
    #[error("This disk cannot be used in a RAID array")]
    #[serde(rename = "disk_internal")]
    DiskInternal,
    #[error("Disk is busy")]
    #[serde(rename = "disk_busy")]
    DiskBusy,
    #[error("RAID array creation failed")]
    #[serde(rename = "create_failed")]
    CreateFailed,
    #[error("The number of disks is too high (basic)")]
    #[serde(rename = "create_too_many_members")]
    CreateTooManyMembers,
    #[error("The number of disks is too small")]
    #[serde(rename = "create_not_enough_members")]
    CreateNotEnoughMembers,
    #[error("The number of disks is incorrect (raid10)")]
    #[serde(rename = "create_bad_member_count")]
    CreateBadMemberCount,
    #[error("This type of RAID array does not support synchronization")]
    #[serde(rename = "sync_action_bad_level")]
    SynActionBadLevel,
    #[error("This RAID array is being resynchronized/restored")]
    #[serde(rename = "sync_action_array_busy")]
    SyncActionArrayBusy,
    #[error("It is not possible to force resynchronization manually")]
    #[serde(rename = "sync_action_bad_action")]
    SyncActionBadAction,
    #[error("This action has been denied")]
    #[serde(rename = "sync_action_failed")]
    SyncActionFailed,
    #[error("Check interval is too long")]
    #[serde(rename = "check_interval_too_large")]
    CheckIntervalTooLarge,
    #[error("This check interval is not supported")]
    #[serde(rename = "check_interval_not_supported")]
    CheckIntervalNotSupported,
    #[error("This type of RAID array does not allow member removal")]
    #[serde(rename = "remove_bad_level")]
    RemoveBadLevel,
    #[error("Not enough active members to allow removal of a member")]
    #[serde(rename = "remove_not_enough_active")]
    RemoveNotEnoughActive,
    #[error("Failure to remove a member")]
    #[serde(rename = "remove_failed")]
    RemoveFailed,
    #[error("Too many new members")]
    #[serde(rename = "add_too_many")]
    AddToMany,
    #[error("One of the members is too small to be added to this array")]
    #[serde(rename = "add_member_too_small")]
    AddMemberTooSmall,
    #[error("Failed to add member")]
    #[serde(rename = "add_failed")]
    AddFailed,
    #[error("Unable to examine member data")]
    #[serde(rename = "member_examine_data_failed")]
    MemberExamineDataFailed,
    #[error("Minimum sync speed is more important than maximum speed")]
    #[serde(rename = "sync_speed_min_greaterthan_max")]
    SyncSpeedMinGreaterThanMax,
    #[error("The minimum sync speed is too high")]
    #[serde(rename = "sync_speed_min_toohigh")]
    SyncSpeedMinTooHigh,
    #[error("The maximum sync speed it too high")]
    #[serde(rename = "sync_speed_max_toohigh")]
    SyncSpeedMaxTooHigh,
    #[error("The minimum sync speed is too low")]
    #[serde(rename = "sync_speed_min_toolow")]
    SyncSpeedMinTooLow,
    #[error("The maximum sync speed is too low")]
    #[serde(rename = "sync_speed_max_toolow")]
    SyncSpeedMaxTooLow,
    #[error("Error changing synchronization speed")]
    #[serde(rename = "sync_speed_set_failed")]
    SyncSpeedSetFailed,
    #[error("RAID level migration not possible")]
    #[serde(rename = "grow_bad_level")]
    GrowBadLevel,
    #[error("Not enough disks for expansion")]
    #[serde(rename = "grow_not_enough_disks")]
    GrowNotEnoughDisks,
    #[error("Expansion failed")]
    #[serde(rename = "grow_failed")]
    GrowFailed,
    #[error("Cannot extend a busy RAID array")]
    #[serde(rename = "grow_array_busy")]
    GrowArrayBusy,
    #[error("One of the members is too small to expand the raid array")]
    #[serde(rename = "grow_member_too_small")]
    GrowMemberTooSmall,
    #[error("One or more members could not be rescanned")]
    #[serde(rename = "rescan_member_failed")]
    RescanMemberFailed,
    #[error("Cannot add out-of-sync disks when the array is busy")]
    #[serde(rename = "add_spares_busy")]
    AddSparesBusy,
    #[error("No out-of-sync member detected")]
    #[serde(rename = "add_spares_nospares")]
    AddSparesNoSpares,
    #[error("The RAID Array is full and cannot add an out of sync member")]
    #[serde(rename = "add_spares_complete")]
    AddSparesComplete,
    #[error("Failed to add out-of-sync disks")]
    #[serde(rename = "add_spares_failed")]
    AddSparesFailed,
}

#[derive(Debug, Clone, Deserialize)]
pub enum RaidArrayState {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Deserialize)]
pub enum RaidArrayLevel {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "raid0")]
    Raid0,
    #[serde(rename = "raid1")]
    Raid1,
    #[serde(rename = "raid5")]
    Raid5,
    #[serde(rename = "raid10")]
    Raid10
}

#[derive(Debug, Clone, Deserialize)]
pub enum RaidArraySyncAction {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "resync")]
    ReSync,
    #[serde(rename = "recover")]
    ReCover,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "repair")]
    Repair,
    #[serde(rename = "reshape")]
    ReShape,
    #[serde(rename = "frozen")]
    Frozen,
}

#[derive(Debug, Clone, Deserialize)]
pub enum RaidArraySysFSState {
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "read_auto")]
    ReadAuto,
    #[serde(rename = "clean")]
    Clean,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "write_pending")]
    WritePending,
    #[serde(rename = "active_idle")]
    ActiveIdle,
}

#[derive(Debug, Clone, Deserialize)]
pub enum RaidMemberRole {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "faulty")]
    Faulty,
    #[serde(rename = "spare")]
    Spare,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RaidArray {
    pub id: i32,
    pub state: RaidArrayState,
    pub name: String,
    pub level: RaidArrayLevel,
    pub disk_id: i32,
    pub uuid: String,
    pub sync_action: RaidArraySyncAction,
    pub sysfs_state: RaidArraySysFSState,
    pub array_size: i32,
    pub raid_disks: i32,
    pub sync_speed: i32,
    pub sync_completed_pos: i32,
    pub sync_completed_end: i32,
    pub sync_completed_percent: i32,
    pub check_interval: i32,
    pub last_check: i32,
    pub next_check: i32,
    pub degraded: bool,
    pub members: Vec<RaidMember>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RaidMember {
    pub id: i32,
    pub array_id: i32,
    pub role: RaidMemberRole,
    pub set_name: String,
    pub set_uuid: String,
    pub dev_uuid: String,
    pub device_location: String,
    pub total_bytes: i32,
    pub active_device: i32,
    pub corrected_read_errors: i32,
    pub sct_erc_supported: bool,
    pub sct_erc_enabled: bool,
    pub disk: RaidDisk,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RaidDisk {
    pub model: String,
    pub serial: String,
    pub firmware: String,
    pub temp: i32,
}

pub struct RaidArrayResponse(pub ResponseResult<RaidArray>);

pub trait StorageRaidCalls<T: HttpClient> {
    async fn get_raid_arrays(&self, client: &T, session: &str) -> RaidArrayResponse;
}