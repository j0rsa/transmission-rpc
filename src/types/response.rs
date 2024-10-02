use std::{
    collections::HashMap,
    net::IpAddr,
};

use base64::{
    Engine as _,
    engine::general_purpose::STANDARD as base64,
};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::de::{Error as _, Deserializer};
use serde::Deserialize;
use serde_repr::*;

use crate::types::request::Priority;
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Deserialize_repr)]
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
    #[serde(deserialize_with = "from_ts_option", default)]
    pub activity_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub added_date: Option<DateTime<Utc>>,
    /// "An array of `pieceCount` numbers representing the number of connected peers that have each
    /// piece, or -1 if we already have the piece ourselves."
    /// Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17).
    pub availability: Option<Vec<i16>>,
    pub bandwidth_priority: Option<Priority>,
    pub comment: Option<String>,
    pub corrupt_ever: Option<u64>,
    pub creator: Option<String>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub date_created: Option<DateTime<Utc>>,
    pub desired_available: Option<u64>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub done_date: Option<DateTime<Utc>>,
    pub download_dir: Option<String>,
    pub downloaded_ever: Option<u64>,
    pub download_limit: Option<u64>,
    pub download_limited: Option<bool>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub edit_date: Option<DateTime<Utc>>,
    pub error: Option<ErrorType>,
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub eta_idle: Option<i64>,
    pub group: Option<String>,
    pub hash_string: Option<String>,
    pub have_unchecked: Option<u64>,
    pub have_valid: Option<u64>,
    pub honors_session_limits: Option<bool>,
    pub id: Option<i64>,
    pub is_finished: Option<bool>,
    pub is_private: Option<bool>,
    pub is_stalled: Option<bool>,
    pub labels: Option<Vec<String>>,
    pub left_until_done: Option<i64>,
    pub magnet_link: Option<String>,
    /// `DateTime::UNIX_EPOCH` if never manually announced.
    #[serde(deserialize_with = "from_ts_option", default)]
    pub manual_announce_time: Option<DateTime<Utc>>,
    pub max_connected_peers: Option<u16>,
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<u16>,
    pub peers: Option<Vec<Peer>>,
    pub peers_connected: Option<i64>,
    pub peers_from: Option<PeersFrom>,
    pub peers_getting_from_us: Option<i64>,
    pub peers_sending_to_us: Option<i64>,
    pub percent_complete: Option<f32>,
    pub percent_done: Option<f32>,
    /// `Pieces` is a wrapper for `Vec<u8>`.
    pub pieces: Option<Pieces>,
    pub piece_count: Option<u64>,
    pub piece_size: Option<u64>,
    #[serde(rename = "primary-mime-type")]
    pub primary_mime_type: Option<String>,
    pub queue_position: Option<usize>,
    pub rate_download: Option<i64>,
    pub rate_upload: Option<i64>,
    pub recheck_progress: Option<f32>,
    pub seconds_downloading: Option<u64>,
    pub seconds_seeding: Option<i64>,
    pub seed_idle_limit: Option<u64>, // Can this be negative?
    pub seed_idle_mode: Option<IdleMode>,
    pub seed_ratio_limit: Option<f32>,
    pub seed_ratio_mode: Option<RatioMode>,
    pub sequential_download: Option<bool>,
    pub size_when_done: Option<i64>,
    #[serde(deserialize_with = "from_ts_option", default)]
    pub start_date: Option<DateTime<Utc>>,
    pub status: Option<TorrentStatus>,
    pub torrent_file: Option<String>,
    pub total_size: Option<i64>,
    pub trackers: Option<Vec<Trackers>>,
    pub tracker_list: Option<String>,
    pub tracker_stats: Option<Vec<TrackerStat>>,
    pub upload_ratio: Option<f32>,
    pub uploaded_ever: Option<i64>,
    pub upload_limit: Option<u64>, // Can this be negative?
    pub upload_limited: Option<bool>,
    pub files: Option<Vec<File>>,
    /// for each file in files, whether or not they will be downloaded (0 or 1)
    pub wanted: Option<Vec<i8>>, // TODO: Deserialize from bool -> i8 (or u8 maybe?) to account for
                                 // TODO: 4.0.0 and 4.0.1
    pub webseeds: Option<Vec<String>>,
    pub webseeds_sending_to_us: Option<u16>,
    pub priorities: Option<Vec<Priority>>,
    pub file_stats: Option<Vec<FileStat>>,
    #[serde(rename = "file-count")]
    pub file_count: Option<usize>,
}

fn from_ts_option<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: i64 = Deserialize::deserialize(deserializer)?;
    // The transmission rpc server responds with 0 or -1 (in the case of manualAnnounceTime) when
    // the date is unset or invalid.
    // Consolidate any response <= 0 as UNIX_EPOCH to denote these cases.
    if ts <= 0 {
        return Ok(Some(DateTime::UNIX_EPOCH));
    }
    Ok(DateTime::<Utc>::from_timestamp(ts, 0))
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
    pub scrape: String,
    /// `the first label before the public suffix in the announce URL's host. eg.
    /// "https://www.example.co.uk/announce"'s sitename is "example"`
    /// Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17)
    #[serde(default)]
    pub sitename: String,
    pub tier: usize,
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
    pub priority: Priority,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub address: IpAddr, // FIXME? serde doesn't like simplified ipv6 addresses
                         // FIXME? (does transmission emit simplified ipv6? eg. "::1")
    pub client_name: String,
    pub client_is_choked: bool,
    pub client_is_interested: bool,
    pub flag_str: String,
    pub is_downloading_from: bool,
    pub is_encrypted: bool,
    pub is_incoming: bool,
    pub is_uploading_to: bool,
    #[serde(rename = "isUTP")]
    pub is_utp: bool,
    pub peer_is_choked: bool,
    pub peer_is_interested: bool,
    pub port: u16,
    pub progress: f32,
    pub rate_to_client: u64, // (B/s)
    pub rate_to_peer: u64, // (B/s)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeersFrom {
    pub from_cache: u16,
    pub from_dht: u16,
    pub from_incoming: u16,
    pub from_lpd: u16,
    pub from_ltep: u16,
    pub from_pex: u16,
    pub from_tracker: u16,
}

#[derive(Clone, Default)]
pub struct Pieces {
    pub bitfield: Vec<u8>,
}

impl<'de> Deserialize<'de> for Pieces {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded: &str = Deserialize::deserialize(deserializer)?;
        let bitfield = base64.decode(encoded).map_err(D::Error::custom)?;
        Ok(Self { bitfield })
    }
}

impl std::ops::Deref for Pieces {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bitfield
    }
}

impl std::fmt::Debug for Pieces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[derive(Deserialize_repr, Debug, Copy, Clone, PartialEq)]
#[repr(i8)]
pub enum IdleMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

#[derive(Deserialize_repr, Debug, Copy, Clone, PartialEq)]
#[repr(i8)]
pub enum RatioMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackerStat {
    pub announce_state: TrackerState,
    pub announce: String,
    pub download_count: i64,
    pub has_announced: bool,
    pub has_scraped: bool,
    pub host: String,
    pub id: Id,
    pub is_backup: bool,
    pub last_announce_peer_count: i64,
    pub last_announce_result: String,
    #[serde(deserialize_with = "from_ts")]
    pub last_announce_start_time: DateTime<Utc>,
    pub last_announce_succeeded: bool,
    #[serde(deserialize_with = "from_ts")]
    pub last_announce_time: DateTime<Utc>,
    pub last_announce_timed_out: bool,
    pub last_scrape_result: String,
    #[serde(deserialize_with = "from_ts")]
    pub last_scrape_start_time: DateTime<Utc>,
    pub last_scrape_succeeded: bool,
    #[serde(deserialize_with = "from_ts")]
    pub last_scrape_time: DateTime<Utc>,
    pub last_scrape_timed_out: bool,
    pub leecher_count: i64,
    #[serde(deserialize_with = "from_ts")]
    pub next_announce_time: DateTime<Utc>,
    #[serde(deserialize_with = "from_ts")]
    pub next_scrape_time: DateTime<Utc>,
    pub scrape_state: TrackerState,
    pub scrape: String,
    pub seeder_count: i64,
    /// `the first label before the public suffix in the announce URL's host. eg.
    /// "https://www.example.co.uk/announce"'s sitename is "example"`
    /// Added in Transmission 4.0.0 (`rpc-version-semver`: 5.3.0, `rpc-version`: 17)
    #[serde(default)]
    pub sitename: String,
    pub tier: usize,
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(i8)]
pub enum TrackerState {
    Inactive = 0,
    Waiting = 1,
    Queued = 2,
    Active = 3,
}

#[derive(Deserialize, Debug)]
pub struct Nothing {}
impl RpcResponseArgument for Nothing {}

#[derive(Debug)]
pub enum TorrentAddedOrDuplicate {
    TorrentDuplicate(Torrent),
    TorrentAdded(Torrent),
    Error,
}

impl RpcResponseArgument for TorrentAddedOrDuplicate {}

impl<'de> Deserialize<'de> for TorrentAddedOrDuplicate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let mut res: HashMap<String, Torrent> = Deserialize::deserialize(deserializer)?;

        let added = res.remove("torrent-added");
        let duplicate = res.remove("torrent-duplicate");
        match (added, duplicate) {
            (Some(torrent), None) => Ok(TorrentAddedOrDuplicate::TorrentAdded(torrent)),
            (None, Some(torrent)) => Ok(TorrentAddedOrDuplicate::TorrentDuplicate(torrent)),
            _ => Ok(TorrentAddedOrDuplicate::Error),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TorrentRenamePath {
    pub path: Option<String>,
    pub name: Option<String>,
    pub id: Option<i64>,
}
impl RpcResponseArgument for TorrentRenamePath {}

#[cfg(test)]
mod tests {
    use crate::types::{Result, RpcResponse, TorrentAddedOrDuplicate};
    use serde_json;
    use serde_json::Value;

    #[test]
    fn test_torrent_added_failure_with_torrent_added_or_duplicate() {
        let v: RpcResponse<TorrentAddedOrDuplicate> =
            serde_json::from_str(torrent_added_failure()).expect("Failure expected");
        println!("{v:#?}");
        assert!(!v.is_ok());
    }

    #[test]
    fn test_torrent_added_success_with_torrent_added_or_duplicate() -> Result<()> {
        let v: RpcResponse<TorrentAddedOrDuplicate> =
            serde_json::from_str(torrent_added_success())?;
        println!("{v:#?}");
        Ok(())
    }

    #[test]
    fn test_torrent_added_success_with_value() -> Result<()> {
        let v: Value = serde_json::from_str(torrent_added_success())?;
        println!("{v:?} {}", serde_json::to_string_pretty(&v).expect(""));
        Ok(())
    }

    #[test]
    fn test_torrent_added_failure_with_value() -> Result<()> {
        let v: Value = serde_json::from_str(torrent_added_failure())?;
        println!("{v:?} {}", serde_json::to_string_pretty(&v).expect(""));
        Ok(())
    }

    fn torrent_added_success() -> &'static str {
        r#"
        {
            "arguments": {
                "torrent-added": {
                    "hashString": "bbdaece7c8daa85e1619469ab25d422a612cf923",
                    "id": 2,
                    "name": "toto.torrent"}
                },
            "result": "success"
        }
        "#
    }

    fn torrent_added_failure() -> &'static str {
        r#"
        {
            "arguments": {},
            "result": "download directory path is not absolute"
        }
        "#
    }
}
