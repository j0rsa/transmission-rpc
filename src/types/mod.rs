
mod request;
mod response;
mod entity;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) use self::request::SessionGet;

pub use self::response::RpcResponse;

pub use self::entity::BasicAuth;
pub use self::entity::SessionInfo;