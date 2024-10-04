mod request;
mod response;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

pub(crate) use self::request::RpcRequest;
pub use self::request::{
    ArgumentFields, Id, Priority, SessionSetArgs, TorrentAction, TorrentAddArgs, TorrentGetField,
    TorrentRenamePathArgs, TorrentSetArgs, TrackerList,
};

pub use self::response::{
    BlocklistUpdate, ErrorType, File, FileStat, FreeSpace, IdleMode, Nothing, Peer, PeersFrom,
    PortTest, RatioMode, RpcResponse, RpcResponseArgument, SessionClose, SessionGet, SessionSet,
    SessionStats, Torrent, TorrentAddedOrDuplicate, TorrentRenamePath, TorrentStatus, Torrents,
    Trackers, TrackerStat, TrackerState,
};
