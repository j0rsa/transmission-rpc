mod request;
mod response;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

pub(crate) use self::request::RpcRequest;
pub use self::request::{
    ArgumentFields, Id, IdleMode, Priority, RatioMode, SessionSetArgs, TorrentAction,
    TorrentAddArgs, TorrentGetField, TorrentRenamePathArgs, TorrentSetArgs, TrackerList,
};

pub use self::response::{
    BlocklistUpdate, ErrorType, FreeSpace, IdleMode, Nothing, PortTest, RatioMode, RpcResponse,
    RpcResponseArgument, SessionClose, SessionGet, SessionSet, SessionStats, Torrent,
    TorrentAddedOrDuplicate, TorrentRenamePath, TorrentStatus, Torrents, TrackerState,
};

/// [`Torrent`] field sub-type. You probably won't need to interact with this directly.
pub use self::response::{File, FileStat, Peer, PeersFrom, TrackerStat, Trackers};
