use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VmError {
    InitFail,
    StartFail,
    InvalidParameter,
    NoMemory,
    AlreadyRunning,
    NotRunning,
    TooBig,
    TooSmall,
    FileExist,
    TooManyVm,
    VmNotExist,
    DiskInUse,
    NoAvailableCpu,
    UsbNotExist,
    UsbAlreadyInUse,
    UsbInitFail,
    DiskNotQcow2,
    UnsupportedDiskFormat,
    DiskNotFound,
    EfiFileAlreadyInUse,
    OpenEfiFileFail,
    InternalHttpError,
    InternalSigError,
    InternalJsonError,
    FileCreationFailed,
    IncorrectPermission,
    OpenFileInfo,
    OpenFileForResizing,
    ResizeRawDisk,
    ShutdownVm,
    RestartVm,
    OpeningDiskFile,
    OpeningCdromFile,
    StartWithoutDisk,
    InitializeVmControl,
    SetupVmWithoutDisk,
    BadFormat,
    SaveData,
    StopControl,
    RetrieveDiskInfo,
    AnalyseDiskInfo,
    RetrieveDiskSize,
    RetrieveActualDiskSize,
    RetrieveDiskFormat,
    CreateQcow2Disk,
    ResizeQcow2Disk,
    VmToManyDisks,
    EmptyDiskPath,
    TaskNotFound,
    NotStopped,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vm {
    pub id: i32,
    pub name: String,
    pub disk_path: String,
    pub disk_type: VmDiskType,
    pub cd_path: String,
    pub memory: i32,
    pub vcpus: i32,
    pub status: VmStatus,
    pub enable_screen: bool,
    //TODO: pub bind_usb_ports: Vec<VmSystemInfo>,
    pub enable_cloudinit: bool,
    pub cloudinit_hostname: String,
    pub cloudinit_userdata: String,
    pub mac: String,
    pub os: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmSystemInfo {
    pub total_memory: i32,
    pub used_memory: i32,
    pub total_cpus: i32,
    pub used_cpus: i32,
    pub usb_ports: Vec<String>,
    pub usb_used: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VmDiskType {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "qcow2")]
    Qcow2,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VmStatus {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "stopping")]
    Stopping,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmDistribution {
    pub name: String,
    pub url: String,
    pub hash: String,
    pub os: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmDiskInfo {
    #[serde(rename = "type")]
    pub kind: VmDiskType,
    pub actual_size: i32,
    pub virtual_size: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VmDiskTask {
    pub id: i32,
    #[serde(rename = "type")]
    pub kind: VmDiskTaskType,
    pub done: bool,
    pub error: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VmDiskTaskType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "resize")]
    Resize,
}
