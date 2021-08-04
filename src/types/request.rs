use enum_iterator::IntoEnumIterator;
use serde::Serialize;

#[derive(Serialize, Debug, RustcEncodable)]
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

    pub fn torrent_get(fields: Option<Vec<TorrentGetField>>, ids: Option<Vec<Id>>) -> RpcRequest {
        let string_fields = fields
            .unwrap_or(TorrentGetField::all())
            .iter()
            .map(|f| f.to_str())
            .collect();
        RpcRequest {
            method: String::from("torrent-get"),
            arguments: Some(Args::TorrentGetArgs(TorrentGetArgs {
                fields: Some(string_fields),
                ids,
            })),
        }
    }

    pub fn torrent_remove(ids: Vec<Id>, delete_local_data: bool) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-remove"),
            arguments: Some(Args::TorrentRemoveArgs(TorrentRemoveArgs {
                ids,
                delete_local_data,
            })),
        }
    }

    pub fn torrent_add(add: TorrentAddArgs) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-add"),
            arguments: Some(Args::TorrentAddArgs(add)),
        }
    }

    pub fn torrent_action(action: TorrentAction, ids: Vec<Id>) -> RpcRequest {
        RpcRequest {
            method: action.to_str(),
            arguments: Some(Args::TorrentActionArgs(TorrentActionArgs { ids })),
        }
    }

    pub fn torrent_set_location(ids: Vec<Id>, location: String, move_from: Option<bool>, ) -> RpcRequest {
        RpcRequest {
            method: String::from("torrent-set-location"),
            arguments: Some(Args::TorrentSetLocationArgs(TorrentSetLocationArgs {
                ids,
                location,
                move_from,
            })),
        }
    }
}
pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField {}

#[derive(Serialize, Debug, RustcEncodable, Clone)]
#[serde(untagged)]
pub enum Args {
    TorrentGetArgs(TorrentGetArgs),
    TorrentActionArgs(TorrentActionArgs),
    TorrentRemoveArgs(TorrentRemoveArgs),
    TorrentAddArgs(TorrentAddArgs),
    TorrentSetLocationArgs(TorrentSetLocationArgs),
}

#[derive(Serialize, Debug, RustcEncodable, Clone)]
pub struct TorrentGetArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<Vec<Id>>,
}

impl Default for TorrentGetArgs {
    fn default() -> Self {
        let all_fields = TorrentGetField::into_enum_iter()
            .map(|it| it.to_str())
            .collect();
        TorrentGetArgs {
            fields: Some(all_fields),
            ids: None,
        }
    }
}

#[derive(Serialize, Debug, RustcEncodable, Clone)]
pub struct TorrentActionArgs {
    ids: Vec<Id>,
}
#[derive(Serialize, Debug, RustcEncodable, Clone)]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    #[serde(rename = "delete-local-data")]
    delete_local_data: bool,
}

#[derive(Serialize, Debug, RustcEncodable, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "move")]
    move_from: Option<bool>,
}

#[derive(Serialize, Debug, RustcEncodable, Clone)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Serialize, Debug, RustcEncodable, Clone)]
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

#[derive(Clone, IntoEnumIterator)]
pub enum TorrentGetField {
    Id,
    Addeddate,
    Name,
    HashString,
    Totalsize,
    Error,
    Errorstring,
    Eta,
    Isfinished,
    Isstalled,
    Leftuntildone,
    Metadatapercentcomplete,
    Peersconnected,
    Peersgettingfromus,
    Peerssendingtous,
    Percentdone,
    Queueposition,
    Ratedownload,
    Rateupload,
    Recheckprogress,
    Seedratiomode,
    Seedratiolimit,
    Sizewhendone,
    Status,
    Trackers,
    Files,
    Downloaddir,
    Uploadedever,
    Uploadratio,
    Webseedssendingtous,
    Wanted,
    Priorities,
    Filestats
}

impl TorrentGetField {
    pub fn all() -> Vec<TorrentGetField> {
        TorrentGetField::into_enum_iter().collect()
    }
}

impl TorrentGetField {
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::Id => "id",
            TorrentGetField::Addeddate => "addedDate",
            TorrentGetField::Name => "name",
            TorrentGetField::HashString => "hashString",
            TorrentGetField::Totalsize => "totalSize",
            TorrentGetField::Error => "error",
            TorrentGetField::Errorstring => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::Isfinished => "isFinished",
            TorrentGetField::Isstalled => "isStalled",
            TorrentGetField::Leftuntildone => "leftUntilDone",
            TorrentGetField::Metadatapercentcomplete => "metadataPercentComplete",
            TorrentGetField::Peersconnected => "peersConnected",
            TorrentGetField::Peersgettingfromus => "peersGettingFromUs",
            TorrentGetField::Peerssendingtous => "peersSendingToUs",
            TorrentGetField::Percentdone => "percentDone",
            TorrentGetField::Queueposition => "queuePosition",
            TorrentGetField::Ratedownload => "rateDownload",
            TorrentGetField::Rateupload => "rateUpload",
            TorrentGetField::Recheckprogress => "recheckProgress",
            TorrentGetField::Seedratiomode => "seedRatioMode",
            TorrentGetField::Seedratiolimit => "seedRatioLimit",
            TorrentGetField::Sizewhendone => "sizeWhenDone",
            TorrentGetField::Status => "status",
            TorrentGetField::Trackers => "trackers",
            TorrentGetField::Files => "files",
            TorrentGetField::Downloaddir => "downloadDir",
            TorrentGetField::Uploadedever => "uploadedEver",
            TorrentGetField::Uploadratio => "uploadRatio",
            TorrentGetField::Webseedssendingtous => "webseedsSendingToUs",
            TorrentGetField::Wanted => "wanted",
            TorrentGetField::Priorities => "priorities",
            TorrentGetField::Filestats => "fileStats",
        }.to_string()
    }
}

pub enum TorrentAction {
    Start,
    Stop,
    StartNow,
    Verify,
    Reannounce,
}

impl TorrentAction {
    pub fn to_str(&self) -> String {
        match self {
            TorrentAction::Start => "torrent-start",
            TorrentAction::Stop => "torrent-stop",
            TorrentAction::StartNow => "torrent-start-now",
            TorrentAction::Verify => "torrent-verify",
            TorrentAction::Reannounce => "torrent-reannounce",
        }.to_string()
    }
}
