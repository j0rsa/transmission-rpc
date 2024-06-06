use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Debug)]
pub struct RpcRequest {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Args>,
}

impl RpcRequest {
    pub fn session_set(args: SessionSetArgs) -> RpcRequest {
        RpcRequest {
            method: String::from("session-set"),
            arguments: Some(Args::SessionSet(args)),
        }
    }

    pub fn session_get() -> RpcRequest {
        RpcRequest {
            method: String::from("session-get"),
            arguments: None,
        }
    }

    pub fn session_stats() -> RpcRequest {
        RpcRequest {
            method: String::from("session-stats"),
            arguments: None,
        }
    }

    pub fn session_close() -> RpcRequest {
        RpcRequest {
            method: String::from("session-close"),
            arguments: None,
        }
    }

    pub fn blocklist_update() -> RpcRequest {
        RpcRequest {
            method: String::from("blocklist-update"),
            arguments: None,
        }
    }

    pub fn free_space(path: String) -> RpcRequest {
        RpcRequest {
            method: String::from("free-space"),
            arguments: Some(Args::FreeSpace(FreeSpaceArgs { path })),
        }
    }

    pub fn port_test() -> RpcRequest {
        RpcRequest {
            method: String::from("port-test"),
            arguments: None,
        }
    }

    pub fn torrent_get(fields: Option<Vec<TorrentGetField>>, ids: Option<Vec<Id>>) -> RpcRequest {
        let string_fields = fields
            .unwrap_or_else(|| all::<TorrentGetField>().collect())
            .iter()
            .map(TorrentGetField::to_str)
            .collect();
        RpcRequest {
            method: String::from("torrent-get"),
            arguments: Some(Args::TorrentGet(TorrentGetArgs {
                fields: Some(string_fields),
                ids,
            })),
        }
    }

    pub fn torrent_set(mut args: TorrentSetArgs, ids: Option<Vec<Id>>) -> RpcRequest {
        args.ids = ids;
        RpcRequest {
            method: String::from("torrent-set"),
            arguments: Some(Args::TorrentSet(args)),
        }
    }

    pub fn torrent_remove(ids: Vec<Id>, delete_local_data: bool) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-remove"),
            arguments: Some(Args::TorrentRemove(TorrentRemoveArgs {
                ids,
                delete_local_data,
            })),
        }
    }

    pub fn torrent_add(add: TorrentAddArgs) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-add"),
            arguments: Some(Args::TorrentAdd(add)),
        }
    }

    pub fn torrent_action(action: TorrentAction, ids: Vec<Id>) -> RpcRequest {
        RpcRequest {
            method: action.to_str(),
            arguments: Some(Args::TorrentAction(TorrentActionArgs { ids })),
        }
    }

    pub fn torrent_set_location(
        ids: Vec<Id>,
        location: String,
        move_from: Option<bool>,
    ) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-set-location"),
            arguments: Some(Args::TorrentSetLocation(TorrentSetLocationArgs {
                ids,
                location,
                move_from,
            })),
        }
    }

    pub fn torrent_rename_path(ids: Vec<Id>, path: String, name: String) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-rename-path"),
            arguments: Some(Args::TorrentRenamePath(TorrentRenamePathArgs {
                ids,
                path,
                name,
            })),
        }
    }
}
pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField {}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Args {
    FreeSpace(FreeSpaceArgs),
    SessionSet(SessionSetArgs),
    TorrentGet(TorrentGetArgs),
    TorrentAction(TorrentActionArgs),
    TorrentRemove(TorrentRemoveArgs),
    TorrentAdd(TorrentAddArgs),
    TorrentSet(TorrentSetArgs),
    TorrentSetLocation(TorrentSetLocationArgs),
    TorrentRenamePath(TorrentRenamePathArgs),
}

#[derive(Serialize, Debug, Clone)]
pub struct FreeSpaceArgs {
    path: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct SessionSetArgs {
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt-speed-down")]
    pub alt_speed_down: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "alt-speed-enabled")]
    pub alt_speed_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "alt-speed-time-begin"
    )]
    pub alt_speed_time_begin: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "alt-speed-time-day")]
    pub alt_speed_time_day: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "alt-speed-time-enabled"
    )]
    pub alt_speed_time_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "alt-speed-time-end")]
    pub alt_speed_time_end: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "alt-speed-up")]
    pub alt_speed_up: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "blocklist-enabled")]
    pub blocklist_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "blocklist-url")]
    pub blocklist_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "cache-size-mb")]
    pub cache_size_mb: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "default-trackers")]
    pub default_trackers: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "dht-enabled")]
    pub dht_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "download-dir")]
    pub download_dir: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "download-dir-free-space"
    )]
    pub download_dir_free_space: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "download-queue-enabled"
    )]
    pub download_queue_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "download-queue-size"
    )]
    pub download_queue_size: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "idle-seeding-limit-enabled"
    )]
    pub idle_seeding_limit_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "idle-seeding-limit")]
    pub idle_seeding_limit: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "incomplete-dir-enabled"
    )]
    pub incomplete_dir_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "incomplete-dir")]
    pub incomplete_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "lpd-enabled")]
    pub lpd_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "peer-limit-global")]
    pub peer_limit_global: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "peer-limit-per-torrent"
    )]
    pub peer_limit_per_torrent: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "peer-port-random-on-start"
    )]
    pub peer_port_random_on_start: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "peer-port")]
    pub peer_port: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "pex-enabled")]
    pub pex_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "port-forwarding-enabled"
    )]
    pub port_forwarding_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "queue-stalled-enabled"
    )]
    pub queue_stalled_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "queue-stalled-minutes"
    )]
    pub queue_stalled_minutes: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "rename-partial-files"
    )]
    pub rename_partial_files: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "script-torrent-added-enabled"
    )]
    pub script_torrent_added_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "script-torrent-added-filename"
    )]
    pub script_torrent_added_filename: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "script-torrent-done-enabled"
    )]
    pub script_torrent_done_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "script-torrent-done-filename"
    )]
    pub script_torrent_done_filename: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "script-torrent-done-seeding-enabled"
    )]
    pub script_torrent_done_seeding_enabled: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "script-torrent-done-seeding-filename"
    )]
    pub script_torrent_done_seeding_filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "seed-queue-enabled")]
    pub seed_queue_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "seed-queue-size")]
    pub seed_queue_size: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "speed-limit-down-enabled"
    )]
    pub speed_limit_down_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "speed-limit-down")]
    pub speed_limit_down: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "speed-limit-up-enabled"
    )]
    pub speed_limit_up_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "speed-limit-up")]
    pub speed_limit_up: Option<i32>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "start-added-torrents"
    )]
    pub start_added_torrents: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "trash-original-torrent-files"
    )]
    pub trash_original_torrent_files: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "utp-enabled")]
    pub utp_enabled: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentGetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<Id>>,
}

impl Default for TorrentGetArgs {
    fn default() -> Self {
        let all_fields = all::<TorrentGetField>().map(|it| it.to_str()).collect();
        TorrentGetArgs {
            fields: Some(all_fields),
            ids: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentActionArgs {
    ids: Vec<Id>,
}
#[derive(Serialize, Debug, Clone)]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    #[serde(rename = "delete-local-data")]
    delete_local_data: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "move")]
    move_from: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentRenamePathArgs {
    ids: Vec<Id>,
    path: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Priority {
    Low = -1,
    Normal = 0,
    High = 1,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct TorrentAddArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "download-dir")]
    pub download_dir: Option<String>,
    /// Either "filename" OR "metainfo" MUST be included
    /// semi-optional
    /// filename or URL of the .torrent file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// semi-optional
    /// base64-encoded .torrent content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "peer-limit")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<Priority>,
    /// list of indices of files to be downloaded
    /// to ignore some files, put their indices in files_unwanted, otherwise
    /// they will still be downloaded
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-wanted")]
    pub files_wanted: Option<Vec<i32>>,
    /// list of indices of files not to download
    #[serde(skip_serializing_if = "Option::is_none", rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with high priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-high")]
    pub priority_high: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with low priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-low")]
    pub priority_low: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with normal priority
    #[serde(skip_serializing_if = "Option::is_none", rename = "priority-normal")]
    pub priority_normal: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

#[derive(Clone, Copy, Sequence)]
pub enum TorrentGetField {
    ActivityDate,
    AddedDate,
    BandwidthPriority,
    DoneDate,
    DownloadDir,
    EditDate,
    Error,
    ErrorString,
    Eta,
    FileCount,
    FileStats,
    Files,
    HashString,
    Id,
    IsFinished,
    IsPrivate,
    IsStalled,
    Labels,
    LeftUntilDone,
    MetadataPercentComplete,
    Name,
    PeersConnected,
    PeersGettingFromUs,
    PeersSendingToUs,
    PercentDone,
    Priorities,
    QueuePosition,
    RateDownload,
    RateUpload,
    RecheckProgress,
    SecondsSeeding,
    SeedRatioLimit,
    SeedRatioMode,
    SizeWhenDone,
    Status,
    TorrentFile,
    TotalSize,
    Trackers,
    TrackerList,
    TrackerStats,
    UploadRatio,
    UploadedEver,
    Wanted,
    WebseedsSendingToUs,
}

impl TorrentGetField {
    #[must_use]
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::ActivityDate => "activityDate",
            TorrentGetField::AddedDate => "addedDate",
            TorrentGetField::BandwidthPriority => "bandwidthPriority",
            TorrentGetField::DoneDate => "doneDate",
            TorrentGetField::DownloadDir => "downloadDir",
            TorrentGetField::EditDate => "editDate",
            TorrentGetField::Error => "error",
            TorrentGetField::ErrorString => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::FileCount => "file-count",
            TorrentGetField::FileStats => "fileStats",
            TorrentGetField::Files => "files",
            TorrentGetField::HashString => "hashString",
            TorrentGetField::Id => "id",
            TorrentGetField::IsFinished => "isFinished",
            TorrentGetField::IsPrivate => "isPrivate",
            TorrentGetField::IsStalled => "isStalled",
            TorrentGetField::Labels => "labels",
            TorrentGetField::LeftUntilDone => "leftUntilDone",
            TorrentGetField::MetadataPercentComplete => "metadataPercentComplete",
            TorrentGetField::Name => "name",
            TorrentGetField::PeersConnected => "peersConnected",
            TorrentGetField::PeersGettingFromUs => "peersGettingFromUs",
            TorrentGetField::PeersSendingToUs => "peersSendingToUs",
            TorrentGetField::PercentDone => "percentDone",
            TorrentGetField::Priorities => "priorities",
            TorrentGetField::QueuePosition => "queuePosition",
            TorrentGetField::RateDownload => "rateDownload",
            TorrentGetField::RateUpload => "rateUpload",
            TorrentGetField::RecheckProgress => "recheckProgress",
            TorrentGetField::SecondsSeeding => "secondsSeeding",
            TorrentGetField::SeedRatioLimit => "seedRatioLimit",
            TorrentGetField::SeedRatioMode => "seedRatioMode",
            TorrentGetField::SizeWhenDone => "sizeWhenDone",
            TorrentGetField::Status => "status",
            TorrentGetField::TorrentFile => "torrentFile",
            TorrentGetField::TotalSize => "totalSize",
            TorrentGetField::Trackers => "trackers",
            TorrentGetField::TrackerList => "trackerList",
            TorrentGetField::TrackerStats => "trackerStats",
            TorrentGetField::UploadRatio => "uploadRatio",
            TorrentGetField::UploadedEver => "uploadedEver",
            TorrentGetField::Wanted => "wanted",
            TorrentGetField::WebseedsSendingToUs => "webseedsSendingToUs",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TorrentAction {
    Start,
    Stop,
    StartNow,
    Verify,
    Reannounce,
}

impl TorrentAction {
    #[must_use]
    pub fn to_str(&self) -> String {
        match self {
            TorrentAction::Start => "torrent-start",
            TorrentAction::Stop => "torrent-stop",
            TorrentAction::StartNow => "torrent-start-now",
            TorrentAction::Verify => "torrent-verify",
            TorrentAction::Reannounce => "torrent-reannounce",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct TrackerList(pub Vec<String>);

impl Serialize for TrackerList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.join("\n").serialize(serializer)
    }
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TorrentSetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files-wanted")]
    pub files_wanted: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honors_session_limits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<Id>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-high")]
    pub priority_high: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-low")]
    pub priority_low: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priority-normal")]
    pub priority_normal: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_idle_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_ratio_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_list: Option<TrackerList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_remove: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_replace: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_limited: Option<bool>,
}
