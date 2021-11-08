use serde::Deserialize;
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
pub struct SessionGet {
    #[serde(rename = "blocklist-enabled")]
    pub blocklist_enabled: bool,
    #[serde(rename = "download-dir")]
    pub download_dir: String,
    pub encryption: String,
    #[serde(rename = "rpc-version")]
    pub rpc_version: i32,
    #[serde(rename = "rpc-version-minimum")]
    pub rpc_version_minimum: i32,
    pub version: String,
}
impl RpcResponseArgument for SessionGet {}

#[derive(Deserialize, Debug, Clone)]
pub struct SessionStats {
    #[serde(rename = "torrentCount")]
    pub torrent_count: i32,
    #[serde(rename = "activeTorrentCount")]
    pub active_torrent_count: i32,
    #[serde(rename = "pausedTorrentCount")]
    pub paused_torrent_count: i32,
    #[serde(rename = "downloadSpeed")]
    pub download_speed: i64,
    #[serde(rename = "uploadSpeed")]
    pub upload_speed: i64,
    #[serde(rename = "current-stats")]
    pub current_stats: Stats,
    #[serde(rename = "cumulative-stats")]
    pub cumulative_stats: Stats,
}
impl RpcResponseArgument for SessionStats {}

#[derive(Deserialize, Debug, Clone)]
pub struct BlocklistUpdate {
    #[serde(rename = "blocklist-size")]
    pub blocklist_size: Option<i32>,
}
impl RpcResponseArgument for BlocklistUpdate {}

#[derive(Deserialize, Debug, Clone)]
pub struct PortTest {
    #[serde(rename = "port-is-open")]
    pub port_is_open: bool,
}
impl RpcResponseArgument for PortTest {}

#[derive(Deserialize, Debug, RustcEncodable, Clone)]
pub struct Torrent {
    #[serde(rename = "addedDate")]
    pub added_date: Option<i64>,
    #[serde(rename = "downloadDir")]
    pub download_dir: Option<String>,
    pub error: Option<i64>,
    #[serde(rename = "errorString")]
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub id: Option<i64>,
    #[serde(rename = "isFinished")]
    pub is_finished: Option<bool>,
    #[serde(rename = "isStalled")]
    pub is_stalled: Option<bool>,
    #[serde(rename = "leftUntilDone")]
    pub left_until_done: Option<i64>,
    #[serde(rename = "metadataPercentComplete")]
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    #[serde(rename = "hashString")]
    pub hash_string: Option<String>,
    #[serde(rename = "peersConnected")]
    pub peers_connected: Option<i64>,
    #[serde(rename = "peersGettingFromUs")]
    pub peers_getting_from_us: Option<i64>,
    #[serde(rename = "peersSendingToUs")]
    pub peers_sending_to_us: Option<i64>,
    #[serde(rename = "percentDone")]
    pub percent_done: Option<f32>,
    #[serde(rename = "rateDownload")]
    pub rate_download: Option<i64>,
    #[serde(rename = "rateUpload")]
    pub rate_upload: Option<i64>,
    #[serde(rename = "recheckProgress")]
    pub recheck_progress: Option<f32>,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(rename = "sizeWhenDone")]
    pub size_when_done: Option<i64>,
    pub status: Option<i64>,
    #[serde(rename = "totalSize")]
    pub total_size: Option<i64>,
    pub trackers: Option<Vec<Trackers>>,
    #[serde(rename = "uploadRatio")]
    pub upload_ratio: Option<f32>,
    #[serde(rename = "uploadedEver")]
    pub uploaded_ever: Option<i64>,
    pub files: Option<Vec<File>>,
    /// for each file in files, whether or not they will be downloaded (0 or 1)
    pub wanted: Option<Vec<i8>>,
    /// for each file in files, their download priority (low:-1,normal:0,high:1)
    pub priorities: Option<Vec<i8>>,
    #[serde(rename = "fileStats")]
    pub file_stats: Option<Vec<FileStat>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stats {
    #[serde(rename = "filesAdded")]
    pub files_added: i32,
    #[serde(rename = "downloadedBytes")]
    pub downloaded_bytes: i64,
    #[serde(rename = "uploadedBytes")]
    pub uploaded_bytes: i64,
    #[serde(rename = "secondsActive")]
    pub seconds_active: i64,
    #[serde(rename = "sessionCount")]
    pub session_count: Option<i32>
}

#[derive(Deserialize, Debug, RustcEncodable)]
pub struct Torrents<T> {
    pub torrents: Vec<T>,
}
impl RpcResponseArgument for Torrents<Torrent> {}

#[derive(Deserialize, Debug, RustcEncodable, Clone)]
pub struct Trackers {
    pub id: i32,
    pub announce: String,
}

#[derive(Deserialize, Debug, RustcEncodable, Clone)]
pub struct File {
    pub length: i64,
    #[serde(rename = "bytesCompleted")]
    pub bytes_completed: i64,
    pub name: String,
}

#[derive(Deserialize, Debug, RustcEncodable, Clone)]
pub struct FileStat {
    #[serde(rename = "bytesCompleted")]
    pub bytes_completed: i64,
    pub wanted: bool,
    /// low: -1, normal: 0, high: 1
    pub priority: i8,
}

#[derive(Deserialize, Debug, RustcEncodable)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Deserialize, Debug, RustcEncodable)]
pub struct TorrentAdded {
    #[serde(rename = "torrent-added")]
    pub torrent_added: Option<Torrent>,
}
impl RpcResponseArgument for TorrentAdded {}

#[derive(Deserialize, Debug, RustcEncodable)]
pub struct TorrentRenamePath{
    pub path: String,
    pub name: String,
    pub id: i64

}
impl RpcResponseArgument for TorrentRenamePath {}
