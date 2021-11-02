mod request;
mod response;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

pub use self::request::ArgumentFields;
pub use self::request::Id;
pub(crate) use self::request::RpcRequest;
pub use self::request::TorrentAction;
pub use self::request::TorrentAddArgs;
pub use self::request::TorrentGetField;

pub use self::response::Nothing;
pub use self::response::RpcResponse;
pub(crate) use self::response::RpcResponseArgument;
pub use self::response::SessionGet;
pub use self::response::SessionStats;
pub use self::response::Torrent;
pub use self::response::TorrentAdded;
pub use self::response::Torrents;
