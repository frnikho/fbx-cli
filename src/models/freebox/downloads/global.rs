use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug, Deserialize)]
pub enum DownloadError {
    #[error("No task was found with the given id")]
    #[serde(rename = "task_not_found")]
    TaskNotFound,
    #[error("Attempt to perform an invalid operation")]
    #[serde(rename = "invalid_operation")]
    InvalidOperation,
    #[error("Error with the download file (invalid format ?)")]
    #[serde(rename = "invalid_file")]
    InvalidFile,
    #[error("URL is invalid")]
    #[serde(rename = "invalid_url")]
    InvalidUrl,
    #[error("Method not implemented")]
    #[serde(rename = "not_implemented")]
    NotImplemented,
    #[error("No more memory available to perform the requested action")]
    #[serde(rename = "out_of_memory")]
    OutOfMemory,
    #[error("The task type is invalid")]
    #[serde(rename = "invalid_task_type")]
    InvalidTaskType,
    #[error("The downloader is hibernating")]
    #[serde(rename = "hibernating")]
    Hibernating,
    #[error("This action is only valid for Bittorrent task in stopped or done state")]
    #[serde(rename = "need_bt_stopped_done")]
    NeedBtStoppedDone,
    #[error("Attempt to access an invalid tracker object")]
    #[serde(rename = "bt_tracker_not_found")]
    BtTrackerNotFound,
    #[error("Too many tasks")]
    #[serde(rename = "too_many_tasks")]
    TooManyTasks,
    #[error("Invalid peer address")]
    #[serde(rename = "invalid_address")]
    InvalidAddress,
    #[error("Port conflict when setting config")]
    #[serde(rename = "port_conflict")]
    PortConflict,
    #[error("Invalid priority")]
    #[serde(rename = "invalid_priority")]
    InvalidPriority,
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    InternalError,
    #[error("Failed to initialize task context file (need to check disk)")]
    #[serde(rename = "ctx_file_error")]
    CtxFileError,
    #[error("Same task already exists")]
    #[serde(rename = "exists")]
    Exits,
    #[error("Incoming port is not available for this customer")]
    #[serde(rename = "port_outside_range")]
    PortOutsideRange,
}

#[derive(Debug, Deserialize)]
pub enum DownloadTaskError {
    None,
    Internal,
    DiskFull,
    Unknown,
    ParseError,
    #[serde(alias = "http_300")]
    Http300,
    #[serde(alias = "http_400")]
    Http400,
    #[serde(alias = "http_500")]
    Http500,
}

#[derive(Error, Clone, Debug, Serialize)]
pub enum DownloadFeedError {
    #[error("No feed was found with the given id")]
    #[serde(rename = "feed_not_found")]
    FeedNotFound,
    #[error("No feed item was found with the given id")]
    #[serde(rename = "item_not_found")]
    ItemNotFound,
    #[error("You are trying to update a feed that is already up to date")]
    #[serde(rename = "feed_is_recent")]
    FeedIsRecent,
    #[error("Internal error")]
    #[serde(rename = "internal_error")]
    Internal
}

#[derive(Clone, Debug, Serialize)]
pub struct Download {
    pub id: i32,
    pub r#type: DownloadType,
    pub name: String,
    pub status: DownloadStatus,
    pub size: i32,
    pub queue_pos: i32,
    pub io_priority: DownloadPriority,
    pub tx_bytes: i32,
    pub rx_bytes: i32,
    pub tx_rates: i32,
    pub rx_rates: i32,
    pub tx_pcts: i8,
    pub error: String, //TODO: replace here
    pub created_ts: i32,
    pub eta: i32,
    pub download_dir: String,
    pub stop_ratio: i32,
    pub archive_password: Option<String>,
    pub info_hash: Option<String>,
    pub piece_length: Option<i32>,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadType {
    #[serde(rename = "bt")]
    BitTorrent,
    #[serde(rename = "nzb")]
    NewsGroup,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "ftp")]
    Ftp,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadStatus {
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "starting")]
    Stating,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "repairing")]
    Repairing,
    #[serde(rename = "extracting")]
    Extracting,
    #[serde(rename = "seeding")]
    Seeding,
    #[serde(rename = "retry")]
    Retry,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High
}

#[derive(Clone, Debug, Serialize)]
pub struct NzbConfigStatus {
    pub status: NzbConfigStatusType,
    pub error: NzbConfigStatusError,
}

#[derive(Clone, Debug, Serialize)]
pub enum NzbConfigStatusType {
    NotChecked,
    Checking,
    Error,
    Ok,
}

#[derive(Clone, Debug, Serialize)]
pub enum NzbConfigStatusError {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "nzb_authentication_required")]
    AuthenticationRequired,
    #[serde(rename = "bad_authentication")]
    BadAuthentication,
    #[serde(rename = "connection_refused")]
    ConnectionRefused,
}

#[derive(Clone, Debug, Serialize)]
pub struct DhtStats {
    pub enabled: bool,
    pub node_count: i32,
    pub enabled_ipv6: bool,
    pub node_count_ipv6: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadStats {
    pub nb_tasks: i32,
    pub nb_tasks_stopped: i32,
    pub nb_tasks_checking: i32,
    pub nb_tasks_queued: i32,
    pub nb_tasks_extracting: i32,
    pub nb_tasks_done: i32,
    pub nb_tasks_repairing: i32,
    pub nb_tasks_seeding: i32,
    pub nb_tasks_downloading: i32,
    pub nb_tasks_error: i32,
    pub nb_tasks_stopping: i32,
    pub nb_tasks_active: i32,
    pub nb_rss: i32,
    pub nb_rss_items_unread: i32,
    pub rx_rate: i32,
    pub tx_rate: i32,
    pub throttling_mode: String,
    pub throttling_is_scheduled: bool,
    pub throttling_rate: Value,
    pub nzb_config_status: NzbConfigStatus,
    pub conn_ready: bool,
    pub nb_peer: i32,
    pub blocklist_entries: i32,
    pub blocklist_hits: i32,
    pub dht_stats: Value,
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadFile {
    pub id: String,
    pub task_id: i32,
    pub path: Option<String>,
    pub filepath: String,
    pub name: String,
    pub mimetype: String,
    pub size: i32,
    pub rx: i32,
    pub status: DownloadFileStatus,
    pub error: String,
    pub priority: DownloadFilePriority,
    pub preview_url: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadFileStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "done")]
    Done
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadFilePriority {
    #[serde(rename = "no_dl")]
    NoDl,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "high")]
    High
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadTracker {
    pub announce: String,
    pub is_backup: bool,
    pub status: DownloadTrackerStatus,
    pub interval: i32,
    pub min_interval: i32,
    pub reannounce_in: i32,
    pub nseeders: i32,
    pub nleechers: i32,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadTrackerStatus {
    #[serde(rename = "unannounced")]
    Unannounced,
    #[serde(rename = "announcing")]
    Announcing,
    #[serde(rename = "announce_failed")]
    AnnounceFailed,
    #[serde(rename = "announced")]
    Announced
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadPeer {
    pub host: String,
    pub port: i32,
    pub state: DownloadPeerState,
    pub origin: DownloadPeerOrigin,
    pub protocol: DownloadPeerProtocol,
    pub client: String,
    pub country_code: String,
    pub tx: i32,
    pub rx: i32,
    pub tx_rate: i32,
    pub rx_rate: i32,
    pub progress: i32,
    pub requests: Vec<i32>
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadPeerState {
    #[serde(rename = "disconnected")]
    Disconnecting,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "handshaking")]
    Handshaking,
    #[serde(rename = "ready")]
    Ready,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadPeerOrigin {
    #[serde(rename = "tracker")]
    Tracker,
    #[serde(rename = "incoming")]
    Incoming,
    #[serde(rename = "dht")]
    Dht,
    #[serde(rename = "pex")]
    PeerExchange,
    #[serde(rename = "user")]
    User,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadPeerProtocol {
    Tcp,
    TcpObfuscated,
    Udp,
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadBlacklistEntry {
    pub host: String,
    pub reason: DownloadBlacklistEntryReason,
    pub expire: i32,
    pub global: bool,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadBlacklistEntryReason {
    #[serde(rename = "not_blacklisted")]
    NotBlacklisted,
    #[serde(rename = "crypto_not_supported")]
    CryptoNotSupported,
    #[serde(rename = "connect_fail")]
    ConnectFailed,
    #[serde(rename = "hs_timeout")]
    HandshakeTimeout,
    #[serde(rename = "hs_failed")]
    HandshakeFailed,
    #[serde(rename = "hs_crypto_failed")]
    HandshakeCryptoFailed,
    #[serde(rename = "hs_crypto_disabled")]
    HandshakeCryptoDisabled,
    #[serde(rename = "torrent_not_found")]
    TorrentNotFound,
    #[serde(rename = "read_failed")]
    ReadFailed,
    #[serde(rename = "write_failed")]
    WriteFailed,
    #[serde(rename = "crap_received")]
    CrapReceived,
    #[serde(rename = "conn_closed")]
    ConnectionClosed,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "blocklist")]
    Blocklist,
    #[serde(rename = "user")]
    User,
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadFeed {
    pub id: i32,
    pub status: DownloadFeedStatus,
    pub url: String,
    pub title: String,
    pub desc: String,
    pub image_url: String,
    pub nb_read: i32,
    pub nb_unread: i32,
    pub auto_download: bool,
    pub fetch_ts: i32,
    pub pub_ts: i32,
    pub error: String,
}

#[derive(Clone, Debug, Serialize)]
pub enum DownloadFeedStatus {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "fetching")]
    Fetching,
    #[serde(rename = "error")]
    Error
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadFeedItem {
    pub id: i32,
    pub feed_id: i32,
    pub title: String,
    pub desc: String,
    pub author: String,
    pub link: String,
    pub is_read: bool,
    pub is_downloaded: bool,
    pub fetch_ts: i32,
    pub pub_ts: i32,
    pub enclosure_ulr: String,
    pub enclosure_type: String,
    pub enclosure_length: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct DownloadConfiguration {
    pub max_downloading_tasks: i32,
    pub download_dir: String,
    pub watch_dir: String,
    pub use_watch_dir: bool,
    pub throttling: DlThrottlingConfig,
    pub news: DlNewsConfig,
    pub bt: DlBtConfig,
    pub feed: DlFeedConfig,
    pub blocklist: DlBlockListConfig,
    pub dns1: String,
    pub dns2: String
}

#[derive(Clone, Debug, Serialize)]
pub struct DlThrottlingConfig {
    pub normal: DlRate,
    pub slow: DlRate,
    pub schedule: Vec<DlThrottlingConfigSchedule>,
    pub mode: DlThrottlingConfigMode,
}

#[derive(Clone, Debug, Serialize)]
pub struct DlRate {
    pub tx_rate: i32,
    pub rx_rate: i32,
}

#[derive(Clone, Debug, Serialize)]
pub enum DlThrottlingConfigSchedule {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "hibernate")]
    Hibernate,
}

#[derive(Clone, Debug, Serialize)]
pub enum DlThrottlingConfigMode {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "hibernate")]
    Hibernate,
    #[serde(rename = "schedule")]
    Schedule
}

#[derive(Clone, Debug, Serialize)]
pub struct DlNewsConfig {
    pub server: String,
    pub port: i32,
    pub ssl: bool,
    pub user: String,
    pub password: String,
    pub nthreads: i32,
    pub auto_repair: bool,
    pub lazy_par2: bool,
    pub auto_extract: bool,
    pub erase_tmp: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct DlBtConfig {
    pub max_peers: i32,
    pub stop_ratio: i32,
    pub crypto_support: DlBtConfigCrypto,
    pub enable_dht: bool,
    pub enable_pex: bool,
    pub announce_timeout: i32,
    pub main_port: i32,
    pub dht_port: i32
}

#[derive(Clone, Debug, Serialize)]
pub enum DlBtConfigCrypto {
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "required")]
    Required,
}

#[derive(Clone, Debug, Serialize)]
pub struct DlFeedConfig {
    pub fetch_interval: i32,
    pub max_items: i32,
}

#[derive(Clone, Debug, Serialize)]
pub struct DlBlockListConfig {
    pub sources: Vec<String>,
}