use enum_iterator::{all, Sequence};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

mod torrent_set;

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct RpcRequest {
    method: String,
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

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct SessionSetArgs {
    pub alt_speed_down: Option<i32>,
    pub alt_speed_enabled: Option<bool>,
    pub alt_speed_time_begin: Option<i32>,
    pub alt_speed_time_day: Option<i32>,
    pub alt_speed_time_enabled: Option<bool>,
    pub alt_speed_time_end: Option<i32>,
    pub alt_speed_up: Option<i32>,
    pub blocklist_enabled: Option<bool>,
    pub blocklist_url: Option<String>,
    pub cache_size_mb: Option<i32>,
    pub default_trackers: Option<String>,
    pub dht_enabled: Option<bool>,
    pub download_dir: Option<String>,
    pub download_dir_free_space: Option<i32>,
    pub download_queue_enabled: Option<bool>,
    pub download_queue_size: Option<i32>,
    pub encryption: Option<String>,
    pub idle_seeding_limit_enabled: Option<bool>,
    pub idle_seeding_limit: Option<i32>,
    pub incomplete_dir_enabled: Option<bool>,
    pub incomplete_dir: Option<String>,
    pub lpd_enabled: Option<bool>,
    pub peer_limit_global: Option<i32>,
    pub peer_limit_per_torrent: Option<i32>,
    pub peer_port_random_on_start: Option<bool>,
    pub peer_port: Option<i32>,
    pub pex_enabled: Option<bool>,
    pub port_forwarding_enabled: Option<bool>,
    pub queue_stalled_enabled: Option<bool>,
    pub queue_stalled_minutes: Option<i32>,
    pub rename_partial_files: Option<bool>,
    pub script_torrent_added_enabled: Option<bool>,
    pub script_torrent_added_filename: Option<String>,
    pub script_torrent_done_enabled: Option<bool>,
    pub script_torrent_done_filename: Option<String>,
    pub script_torrent_done_seeding_enabled: Option<bool>,
    pub script_torrent_done_seeding_filename: Option<String>,
    pub seed_queue_enabled: Option<bool>,
    pub seed_queue_size: Option<i32>,
    #[serde(rename = "seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(rename = "seedRatioLimited")]
    pub seed_ratio_limited: Option<bool>,
    pub speed_limit_down_enabled: Option<bool>,
    pub speed_limit_down: Option<i32>,
    pub speed_limit_up_enabled: Option<bool>,
    pub speed_limit_up: Option<i32>,
    pub start_added_torrents: Option<bool>,
    pub trash_original_torrent_files: Option<bool>,
    pub utp_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentGetArgs {
    fields: Option<Vec<String>>,
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
#[serde(rename_all = "kebab-case")]
pub struct TorrentRemoveArgs {
    ids: Vec<Id>,
    delete_local_data: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct TorrentSetLocationArgs {
    ids: Vec<Id>,
    location: String,
    #[serde(rename = "move")]
    move_from: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TorrentRenamePathArgs {
    ids: Vec<Id>,
    path: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum IdleMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum RatioMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub struct TorrentAddArgs {
    pub cookies: Option<String>,
    pub download_dir: Option<String>,
    /// Either "filename" OR "metainfo" MUST be included
    /// semi-optional
    /// filename or URL of the .torrent file
    pub filename: Option<String>,
    /// semi-optional
    /// base64-encoded .torrent content
    pub metainfo: Option<String>,
    pub paused: Option<bool>,
    pub peer_limit: Option<i64>,
    #[serde(rename = "bandwidthPriority")]
    pub bandwidth_priority: Option<Priority>,
    /// list of indices of files to be downloaded
    /// to ignore some files, put their indices in files_unwanted, otherwise
    /// they will still be downloaded
    pub files_wanted: Option<Vec<i32>>,
    /// list of indices of files not to download
    pub files_unwanted: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with high priority
    pub priority_high: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with low priority
    pub priority_low: Option<Vec<i32>>,
    /// list of indices of files to be downloaded with normal priority
    pub priority_normal: Option<Vec<i32>>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
#[cfg_attr(feature = "tor-get-serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "tor-get-serde", serde(rename_all = "camelCase"))]
pub enum TorrentGetField {
    ActivityDate,
    AddedDate,
    Availability,
    BandwidthPriority,
    Comment,
    CorruptEver,
    Creator,
    DateCreated,
    DesiredAvailable,
    DoneDate,
    DownloadDir,
    DownloadedEver,
    DownloadLimit,
    DownloadLimited,
    EditDate,
    Error,
    ErrorString,
    Eta,
    EtaIdle,
    #[cfg_attr(feature = "tor-get-serde", serde(rename = "file-count"))]
    FileCount,
    FileStats,
    Files,
    Group,
    HashString,
    HaveUnchecked,
    HaveValid,
    HonorsSessionLimits,
    Id,
    IsFinished,
    IsPrivate,
    IsStalled,
    Labels,
    LeftUntilDone,
    MagnetLink,
    ManualAnnounceTime,
    MaxConnectedPeers,
    MetadataPercentComplete,
    Name,
    #[cfg_attr(feature = "tor-get-serde", serde(rename = "peer-limit"))]
    PeerLimit,
    Peers,
    PeersConnected,
    PeersFrom,
    PeersGettingFromUs,
    PeersSendingToUs,
    PercentComplete,
    PercentDone,
    Pieces,
    PieceCount,
    PieceSize,
    Priorities,
    #[cfg_attr(feature = "tor-get-serde", serde(rename = "primary-mime-type"))]
    PrimaryMimeType,
    QueuePosition,
    RateDownload,
    RateUpload,
    RecheckProgress,
    SecondsDownloading,
    SecondsSeeding,
    SeedIdleLimit,
    SeedIdleMode,
    SeedRatioLimit,
    SeedRatioMode,
    SequentialDownload,
    SizeWhenDone,
    StartDate,
    Status,
    TorrentFile,
    TotalSize,
    Trackers,
    TrackerList,
    TrackerStats,
    UploadRatio,
    UploadedEver,
    UploadLimit,
    UploadLimited,
    Wanted,
    Webseeds,
    WebseedsSendingToUs,
}

impl TorrentGetField {
    #[must_use]
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::ActivityDate => "activityDate",
            TorrentGetField::AddedDate => "addedDate",
            TorrentGetField::Availability => "availability",
            TorrentGetField::BandwidthPriority => "bandwidthPriority",
            TorrentGetField::Comment => "comment",
            TorrentGetField::CorruptEver => "corruptEver",
            TorrentGetField::Creator => "creator",
            TorrentGetField::DateCreated => "dateCreated",
            TorrentGetField::DesiredAvailable => "desiredAvailable",
            TorrentGetField::DoneDate => "doneDate",
            TorrentGetField::DownloadDir => "downloadDir",
            TorrentGetField::DownloadedEver => "downloadedEver",
            TorrentGetField::DownloadLimit => "downloadLimit",
            TorrentGetField::DownloadLimited => "downloadLimited",
            TorrentGetField::EditDate => "editDate",
            TorrentGetField::Error => "error",
            TorrentGetField::ErrorString => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::EtaIdle => "etaIdle",
            TorrentGetField::FileCount => "file-count",
            TorrentGetField::FileStats => "fileStats",
            TorrentGetField::Files => "files",
            TorrentGetField::Group => "group",
            TorrentGetField::HashString => "hashString",
            TorrentGetField::HaveUnchecked => "haveUnchecked",
            TorrentGetField::HaveValid => "haveValid",
            TorrentGetField::HonorsSessionLimits => "honorsSessionLimits",
            TorrentGetField::Id => "id",
            TorrentGetField::IsFinished => "isFinished",
            TorrentGetField::IsPrivate => "isPrivate",
            TorrentGetField::IsStalled => "isStalled",
            TorrentGetField::Labels => "labels",
            TorrentGetField::LeftUntilDone => "leftUntilDone",
            TorrentGetField::MagnetLink => "magnetLink",
            TorrentGetField::ManualAnnounceTime => "manualAnnounceTime",
            TorrentGetField::MaxConnectedPeers => "maxConnectedPeers",
            TorrentGetField::MetadataPercentComplete => "metadataPercentComplete",
            TorrentGetField::Name => "name",
            TorrentGetField::PeerLimit => "peer-limit",
            TorrentGetField::Peers => "peers",
            TorrentGetField::PeersConnected => "peersConnected",
            TorrentGetField::PeersFrom => "peersFrom",
            TorrentGetField::PeersGettingFromUs => "peersGettingFromUs",
            TorrentGetField::PeersSendingToUs => "peersSendingToUs",
            TorrentGetField::PercentComplete => "percentComplete",
            TorrentGetField::PercentDone => "percentDone",
            TorrentGetField::Pieces => "pieces",
            TorrentGetField::PieceCount => "pieceCount",
            TorrentGetField::PieceSize => "pieceSize",
            TorrentGetField::Priorities => "priorities",
            TorrentGetField::PrimaryMimeType => "primary-mime-type",
            TorrentGetField::QueuePosition => "queuePosition",
            TorrentGetField::RateDownload => "rateDownload",
            TorrentGetField::RateUpload => "rateUpload",
            TorrentGetField::RecheckProgress => "recheckProgress",
            TorrentGetField::SecondsDownloading => "secondsDownloading",
            TorrentGetField::SecondsSeeding => "secondsSeeding",
            TorrentGetField::SeedIdleLimit => "seedIdleLimit",
            TorrentGetField::SeedIdleMode => "seedIdleMode",
            TorrentGetField::SeedRatioLimit => "seedRatioLimit",
            TorrentGetField::SeedRatioMode => "seedRatioMode",
            TorrentGetField::SequentialDownload => "sequentialDownload",
            TorrentGetField::SizeWhenDone => "sizeWhenDone",
            TorrentGetField::StartDate => "startDate",
            TorrentGetField::Status => "status",
            TorrentGetField::TorrentFile => "torrentFile",
            TorrentGetField::TotalSize => "totalSize",
            TorrentGetField::Trackers => "trackers",
            TorrentGetField::TrackerList => "trackerList",
            TorrentGetField::TrackerStats => "trackerStats",
            TorrentGetField::UploadRatio => "uploadRatio",
            TorrentGetField::UploadedEver => "uploadedEver",
            TorrentGetField::UploadLimit => "uploadLimit",
            TorrentGetField::UploadLimited => "uploadLimited",
            TorrentGetField::Wanted => "wanted",
            TorrentGetField::Webseeds => "webseeds",
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

/// Defines request arguments for the [`torrent_set`] method.
///
/// # Constructors
///
/// * [`TorrentSetArgs::default`] creates a new [`TorrentSetArgs`] instance with all fields set to
/// their default value (`None`).
/// * [`TorrentSetArgs::new`] is an alias for [`TorrentSetArgs::default`].
///
/// # Setters
///
/// These methods are fluent setters, returning a new [`TorrentSetArgs`] instance modifying only
/// the corresponding field while leaving all other fields untouched.
///
/// * [`TorrentSetArgs::bandwidth_priority`]: The torrents' bandwidth [`Priority`].
/// * [`TorrentSetArgs::download_limit`]: Maximum download speed (`KBps`).
/// * [`TorrentSetArgs::download_limited`]: `true` to honor `download_limit`.
/// * [`TorrentSetArgs::files_wanted`]: Indices of file(s) to download.
/// * [`TorrentSetArgs::files_unwanted`]: Indices of file(s) to skip (ie, not download).
/// * [`TorrentSetArgs::group`]: The name of the torrents' bandwidth group.
///     > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17).
/// * [`TorrentSetArgs::honors_session_limits`]: `true` to honor the session's upload limits.
/// * [`TorrentSetArgs::labels`]: A `Vec` of `String` labels to set on the torrent(s).
///     > Added in Transmission 3.00 (`rpc-version-semver` 5.2.0, `rpc-version`: 16).
/// * [`TorrentSetArgs::location`]: The new location of the torrents' content.
/// * [`TorrentSetArgs::peer_limit`]: Maximum number of peers.
/// * [`TorrentSetArgs::priority_high`]: Indices of [`Priority::High`] file(s).
/// * [`TorrentSetArgs::priority_low`]: Indices of [`Priority::Low`] file(s).
/// * [`TorrentSetArgs::priority_normal`]: Indices of [`Priority::Normal`] file(s).
/// * [`TorrentSetArgs::queue_position`]: The new queue position of this torrent `[0..n)`.
/// * [`TorrentSetArgs::seed_idle_limit`]: Torrent-level number of minutes of seeding inactivity
/// before it considered `stalled`.
/// * [`TorrentSetArgs::seed_idle_mode`]: Which seeding inactivity mode ([`IdleMode`]) to use.
/// * [`TorrentSetArgs::seed_ratio_limit`]: Torrent-level seeding ratio.
/// * [`TorrentSetArgs::seed_ratio_mode`]: Which [`RatioMode`] to use.
/// * [`TorrentSetArgs::sequential_download`]: `true` to download the torrent pieces sequentially.
///     > Added in Transmission 4.1.0 (`rpc-version-semver` 5.4.0, `rpc-version`: 18).
/// * [`TorrentSetArgs::tracker_add`]: Add a new tracker url in its own new tier.
///     * *NOTE:* This documentation may be incorrect. The rpc-spec itself is unclear.
///     > ⚠ Deprecated in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17);
///     > prefer `tracker_list` if possible.
/// * [`TorrentSetArgs::tracker_list`]: `TrackerList` of announce urls with an empty element
/// between [tiers](https://www.bittorrent.org/beps/bep_0012.html).
///     > Added in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17).
/// * [`TorrentSetArgs::tracker_remove`]: [`Trackers::id`] of trackers to remove.
///     * *NOTE:* This documentation may be incorrect. The rpc-spec itself is unclear.
///     > ⚠ Deprecated in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17);
///     > prefer `tracker_list` if possible.
/// * [`TorrentSetArgs::tracker_replace`]: Pairs of <[`Trackers::id`]/new announce urls>.
///     * *NOTE:* This documentation may be incorrect. The rpc-spec itself is unclear.
///     * See: transmission/transmission
///     [#3226](https://github.com/transmission/transmission/issues/3226#issuecomment-1411899883).
///     > ⚠ Deprecated in Transmission 4.0.0 (`rpc-version-semver` 5.3.0, `rpc-version`: 17);
///     > prefer `tracker_list` if possible.
/// * [`TorrentSetArgs::upload_limit`]: Maximum upload speed (`KBps`).
/// * [`TorrentSetArgs::upload_limited`]: `true` to honor `upload_limit`.
///
/// # Examples
///
/// With fluent setters:
/// ```
/// use transmission_rpc::types::TorrentSetArgs;
///
/// let args = TorrentSetArgs::default()
///                .seed_ratio_limit(12.34)
///                .labels(vec![String::from("foo"), String::from("bar")])
///                .location(String::from("/a/b/c/d"));
/// ```
///
/// Directly setting struct fields:
/// ```
/// use transmission_rpc::types::TorrentSetArgs;
///
/// let mut args = TorrentSetArgs::default();
/// args.seed_ratio_limit = Some(12.34);
/// args.labels = Some(vec![String::from("foo"), String::from("bar")]);
/// args.location = Some(String::from("/a/b/c/d"));
/// ```
///
/// [`torrent_set`]: crate::TransClient::torrent_set
/// [`Trackers::id`]: super::Trackers::id
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TorrentSetArgs {
    pub bandwidth_priority: Option<Priority>,
    pub download_limit: Option<usize>,
    pub download_limited: Option<bool>,
    #[serde(rename = "files-wanted")]
    pub files_wanted: Option<Vec<usize>>,
    #[serde(rename = "files-unwanted")]
    pub files_unwanted: Option<Vec<usize>>,
    pub group: Option<String>,
    pub honors_session_limits: Option<bool>,
    // Don't expose the `ids` field as it is blindly overwritten by `torrent_set`.
    ids: Option<Vec<Id>>,
    pub labels: Option<Vec<String>>,
    pub location: Option<String>,
    #[serde(rename = "peer-limit")]
    pub peer_limit: Option<u16>,
    #[serde(rename = "priority-high")]
    pub priority_high: Option<Vec<usize>>,
    #[serde(rename = "priority-low")]
    pub priority_low: Option<Vec<usize>>,
    #[serde(rename = "priority-normal")]
    pub priority_normal: Option<Vec<usize>>,
    pub queue_position: Option<usize>,
    pub seed_idle_limit: Option<u16>,
    pub seed_idle_mode: Option<IdleMode>,
    pub seed_ratio_limit: Option<f64>,
    pub seed_ratio_mode: Option<RatioMode>,
    pub sequential_download: Option<bool>,
    pub tracker_add: Option<Vec<String>>,
    pub tracker_list: Option<TrackerList>,
    pub tracker_remove: Option<Vec<String>>,
    pub tracker_replace: Option<Vec<String>>,
    pub upload_limit: Option<usize>,
    pub upload_limited: Option<bool>,
}
