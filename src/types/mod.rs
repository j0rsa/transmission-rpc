mod request;
mod response;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

pub(crate) use self::request::RpcRequest;
pub use self::request::{
    ArgumentFields, Id, TorrentAction, TorrentAddArgs, TorrentGetField, TorrentRenamePathArgs,
    TorrentSetArgs, TrackerList,
};

pub(crate) use self::response::RpcResponseArgument;
pub use self::response::{
    BlocklistUpdate, FreeSpace, Nothing, PortTest, RpcResponse, SessionClose, SessionGet,
    SessionStats, Torrent, TorrentAddedOrDuplicate, TorrentRenamePath, Torrents,
};
