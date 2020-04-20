
mod request;
mod response;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

pub(crate) use self::request::RpcRequestArgument;
pub(crate) use self::request::SessionGet;

pub use self::response::RpcResponse;

pub(crate) use self::response::RpcResponseArgument;
pub use self::response::SessionInfo;