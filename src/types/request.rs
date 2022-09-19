use enum_iterator::{all, Sequence};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct RpcRequest {
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Args>,
}

impl RpcRequest {
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
            .unwrap_or(all::<TorrentGetField>().collect())
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
    TorrentGet(TorrentGetArgs),
    TorrentAction(TorrentActionArgs),
    TorrentRemove(TorrentRemoveArgs),
    TorrentAdd(TorrentAddArgs),
    TorrentSetLocation(TorrentSetLocationArgs),
    TorrentRenamePath(TorrentRenamePathArgs),
}

#[derive(Serialize, Debug, Clone)]
pub struct FreeSpaceArgs {
    path: String,
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

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Serialize, Debug, Default, Clone)]
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
    pub bandwidth_priority: Option<i64>,
    /// list of indices of files to be downloaded
    /// to ignore some files, put their indices in files_unwanted, otherwise they will still be downloaded
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
}

impl Default for TorrentAddArgs {
    fn default() -> Self {
        TorrentAddArgs {
            cookies: None,
            download_dir: None,
            filename: None,
            metainfo: None,
            paused: None,
            peer_limit: None,
            bandwidth_priority: None,
            files_wanted: None,
            files_unwanted: None,
            priority_high: None,
            priority_low: None,
            priority_normal: None,
        }
    }
}

// https://github.com/transmission/transmission/blob/main/docs/rpc-spec.md
#[derive(Clone, Sequence)]
pub enum TorrentGetField {
    ActivityDate,
    AddedDate,
    DoneDate,
    DownloadDir,
    Error,
    ErrorString,
    Eta,
    FileStats,
    Files,
    HashString,
    Id,
    IsFinished,
    IsStalled,
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
    SeedRatioLimit,
    SeedRatioMode,
    SizeWhenDone,
    Status,
    TotalSize,
    Trackers,
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
            TorrentGetField::DoneDate => "doneDate",
            TorrentGetField::DownloadDir => "downloadDir",
            TorrentGetField::Error => "error",
            TorrentGetField::ErrorString => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::FileStats => "fileStats",
            TorrentGetField::Files => "files",
            TorrentGetField::HashString => "hashString",
            TorrentGetField::Id => "id",
            TorrentGetField::IsFinished => "isFinished",
            TorrentGetField::IsStalled => "isStalled",
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
            TorrentGetField::SeedRatioLimit => "seedRatioLimit",
            TorrentGetField::SeedRatioMode => "seedRatioMode",
            TorrentGetField::SizeWhenDone => "sizeWhenDone",
            TorrentGetField::Status => "status",
            TorrentGetField::TotalSize => "totalSize",
            TorrentGetField::Trackers => "trackers",
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
