use serde::Serialize;
use thiserror::Error;

#[derive(Error, Clone, Debug, Serialize)]
pub enum FileSystemError {
    #[error("Invalid object id")]
    #[serde(rename = "invalid_id")]
    InvalidId,
    #[error("File or folder not found")]
    #[serde(rename = "path_not_found")]
    PathNotFound,
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    Internal,
    #[error("The disk is not mounted")]
    #[serde(rename = "disk_unavailable")]
    DiskUnavailable,
    #[error("Invalid request")]
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[error("The conflict mode specified is invalid (see below)")]
    #[serde(rename = "invalid_conflict_mode")]
    InvalidConflictMode,
    #[error("Internal error")]
    #[serde(rename = "exec_failed")]
    ExecFailed,
    #[error("Out of memory")]
    #[serde(rename = "out_of_memory")]
    OutOfMemory,
    #[error("Invalid task id")]
    #[serde(rename = "task_not_found")]
    TaskNotFound,
    #[error("You tried to set an invalid state")]
    #[serde(rename = "invalid_state")]
    InvalidState,
    #[error("This operation cannot be performed on this task")]
    #[serde(rename = "invalid_task_type")]
    InvalidTaskType,
    #[error("The destination file/folder already exists")]
    #[serde(rename = "destination_conflict")]
    DestinationConflict,
    #[error("Access to this file is denied")]
    #[serde(rename = "access_denied")]
    AccessDenied,
    #[error("The destination disk is full")]
    #[serde(rename = "disk_full")]
    DiskFull
}

#[derive(Error, Debug, Clone, Serialize)]
pub enum FileShareError {
    #[error("Invalid object id")]
    #[serde(rename = "invalid_id")]
    InvalidId,
    #[error("File or folder not found")]
    #[serde(rename = "path_not_found")]
    PathNotFound,
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    Internal,
}

#[derive(Error, Debug, Clone, Serialize)]
pub enum FileUploadError {
    #[error("Invalid request")]
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[error("File or folder not found")]
    #[serde(rename = "path_not_found")]
    PathNotFound,
    #[error("Write permission denied in the destination folder")]
    #[serde(rename = "access_denied")]
    AccessDenied,
    #[error("A file with same name already exists")]
    #[serde(rename = "destination_conflict")]
    DestinationConflict,
    #[error("Invalid file upload id")]
    #[serde(rename = "invalid_id")]
    InvalidId,
    #[error("Someone on a side channel as cancelled the upload")]
    #[serde(rename = "cancelled")]
    Cancelled,
    #[error("No upload with this id")]
    #[serde(rename = "noent")]
    NoEntity,
}

#[derive(Debug, Clone, Serialize)]
pub struct FsTask {
    pub id: i32,
    pub r#type: FsTaskType,
    pub state: FsTaskState,
    pub error: FsTaskError,
    pub created_ts: i32,
    pub started_ts: i32,
    pub done_ts: i32,
    pub duration: i32,
    pub progress: i32,
    pub eta: i32,
    pub from: String,
    pub to: String,
    pub nfiles: i32,
    pub nfiles_done: i32,
    pub total_bytes: i32,
    pub total_bytes_done: i32,
    pub curr_bytes_done: i32,
    pub rate: i32,
    pub src: Vec<String>,
    pub dst: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum FsTaskType {
    #[serde(rename = "cat")]
    Cat,
    #[serde(rename = "cp")]
    Cp,
    #[serde(rename = "mv")]
    Mv,
    #[serde(rename = "rm")]
    Rm,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "extract")]
    Extract,
    #[serde(rename = "repair")]
    Repair,
}

#[derive(Debug, Clone, Serialize)]
pub enum FsTaskState {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Serialize)]
pub enum FsTaskError {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "archive_read_failed")]
    ArchiveReadFailed,
    #[serde(rename = "archive_open_failed")]
    ArchiveOpenFailed,
    #[serde(rename = "archive_write_failed")]
    ArchiveWriteFailed,
    #[serde(rename = "chdir_failed")]
    ChangeDirectoryFailed,
    #[serde(rename = "dest_is_not_dir")]
    DestinationIsNotDirectory,
    #[serde(rename = "file_exists")]
    FileAlreadyExist,
    #[serde(rename = "file_not_found")]
    FileNotFound,
    #[serde(rename = "mkdir_failed")]
    CreateDirectoryFailed,
    #[serde(rename = "open_input_failed")]
    OpenInputFailed,
    #[serde(rename = "open_output_failed")]
    OpenOutputFailed,
    #[serde(rename = "opendir_failed")]
    OpenDirectoryFailed,
    #[serde(rename = "overwrite_failed")]
    OverwriteFailed,
    #[serde(rename = "path_too_big")]
    PathTooBig,
    #[serde(rename = "repair_failed")]
    RepairFailed,
    #[serde(rename = "rmdir_failed")]
    RemoveDirectoryFailed,
    #[serde(rename = "same_file")]
    SameFile,
    #[serde(rename = "unlink_failed")]
    UnlinkFailed,
    #[serde(rename = "unsupported_file_type")]
    UnsupportedFileType,
    #[serde(rename = "write_failed")]
    WriteFailed,
    #[serde(rename = "disk_full")]
    DiskFull,
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "invalid_format")]
    InvalidFormat,
    #[serde(rename = "incorrect_password")]
    IncorrectPassword,
    #[serde(rename = "permission_denied")]
    PermissionDenied,
    #[serde(rename = "readlink_failed")]
    ReadlinkFailed,
    #[serde(rename = "symlink_failed")]
    SymlinkFailed,
    #[serde(rename = "copy_into_itself")]
    CopyIntoItself,
    #[serde(rename = "truncate_failed")]
    TruncateFailed,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub mimetype: String,
    pub r#type: FileInfoType,
    pub size: i32,
    pub modification: i32,
    pub index: i32,
    pub link: bool,
    pub target: String,
    pub hidden: bool,
    pub foldercount: i32,
    pub filecount: i32,
}

#[derive(Debug, Clone, Serialize)]
pub enum FileInfoType {
    #[serde(rename = "dir")]
    Directory,
    #[serde(rename = "file")]
    File,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShareLink {
    pub token: String,
    pub path: String,
    pub name: String,
    pub expire: i32,
    pub fullurl: String
}

#[derive(Debug, Clone, Serialize)]
pub struct FileUpload {
    pub id: i32,
    pub size: i32,
    pub uploaded: i32,
    pub status: FileUploadStatus,
    pub start_date: i32,
    pub last_update: i32,
    pub upload_name: String,
    pub dirname: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum FileUploadStatus {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "conflict")]
    Conflict,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "cancelled")]
    Cancelled,
}