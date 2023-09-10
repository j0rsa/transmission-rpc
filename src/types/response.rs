use serde::Deserialize;
use serde_repr::*;

use crate::types::Id;

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T: RpcResponseArgument> {
    pub arguments: T,
    pub result: String,
}

impl<T: RpcResponseArgument> RpcResponse<T> {
    pub fn is_ok(&self) -> bool {
        self.result == "success"
    }
}
pub trait RpcResponseArgument {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionSet {}
impl RpcResponseArgument for SessionSet {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct SessionGet {
    pub blocklist_enabled: bool,
    pub download_dir: String,
    pub encryption: String,
    pub rpc_version: i32,
    pub rpc_version_minimum: i32,
    pub version: String,
}
impl RpcResponseArgument for SessionGet {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionStats {
    pub torrent_count: i32,
    pub active_torrent_count: i32,
    pub paused_torrent_count: i32,
    pub download_speed: i64,
    pub upload_speed: i64,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
}
impl RpcResponseArgument for SessionStats {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionClose {}
impl RpcResponseArgument for SessionClose {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct BlocklistUpdate {
    pub blocklist_size: Option<i32>,
}
impl RpcResponseArgument for BlocklistUpdate {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FreeSpace {
    pub path: String,
    pub size_bytes: i64,
}
impl RpcResponseArgument for FreeSpace {}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PortTest {
    pub port_is_open: bool,
}
impl RpcResponseArgument for PortTest {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum TorrentStatus {
    Stopped = 0,
    QueuedToVerify = 1,
    Verifying = 2,
    QueuedToDownload = 3,
    Downloading = 4,
    QueuedToSeed = 5,
    Seeding = 6,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum ErrorType {
    Ok = 0,
    TrackerWarning = 1,
    TrackerError = 2,
    LocalError = 3,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Torrent {
    pub activity_date: Option<i64>,
    pub added_date: Option<i64>,
    pub done_date: Option<i64>,
    pub download_dir: Option<String>,
    pub edit_date: Option<i64>,
    pub error: Option<ErrorType>,
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub id: Option<i64>,
    pub is_finished: Option<bool>,
    pub is_private: Option<bool>,
    pub is_stalled: Option<bool>,
    pub labels: Option<Vec<String>>,
    pub left_until_done: Option<i64>,
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    pub hash_string: Option<String>,
    pub peers_connected: Option<i64>,
    pub peers_getting_from_us: Option<i64>,
    pub peers_sending_to_us: Option<i64>,
    pub percent_done: Option<f32>,
    pub rate_download: Option<i64>,
    pub rate_upload: Option<i64>,
    pub recheck_progress: Option<f32>,
    pub seconds_seeding: Option<i64>,
    pub seed_ratio_limit: Option<f32>,
    pub size_when_done: Option<i64>,
    pub status: Option<TorrentStatus>,
    pub torrent_file: Option<String>,
    pub total_size: Option<i64>,
    pub trackers: Option<Vec<Trackers>>,
    pub upload_ratio: Option<f32>,
    pub uploaded_ever: Option<i64>,
    pub files: Option<Vec<File>>,
    /// for each file in files, whether or not they will be downloaded (0 or 1)
    pub wanted: Option<Vec<i8>>,
    /// for each file in files, their download priority (low:-1,normal:0,high:1)
    pub priorities: Option<Vec<i8>>,
    pub file_stats: Option<Vec<FileStat>>,
}

impl Torrent {
    /// Get either the ID or the hash string if exist, which are both unique and
    /// can be pass to the API.
    pub fn id(&self) -> Option<Id> {
        self.id
            .map(Id::Id)
            .or_else(|| self.hash_string.clone().map(Id::Hash))
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub files_added: i32,
    pub downloaded_bytes: i64,
    pub uploaded_bytes: i64,
    pub seconds_active: i64,
    pub session_count: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Torrents<T> {
    pub torrents: Vec<T>,
}
impl RpcResponseArgument for Torrents<Torrent> {}

#[derive(Deserialize, Debug, Clone)]
pub struct Trackers {
    pub id: i32,
    pub announce: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub length: i64,
    pub bytes_completed: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileStat {
    pub bytes_completed: i64,
    pub wanted: bool,
    /// low: -1, normal: 0, high: 1
    pub priority: i8,
}

#[derive(Deserialize, Debug)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TorrentAddedOrDuplicate {
    TorrentDuplicate(Torrent),
    TorrentAdded(Torrent),
}
impl RpcResponseArgument for TorrentAddedOrDuplicate {}

#[derive(Deserialize, Debug)]
pub struct TorrentRenamePath {
    pub path: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}
impl RpcResponseArgument for TorrentRenamePath {}
