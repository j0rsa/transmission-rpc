use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    arguments: T,
    result: String
}

#[derive(Deserialize, Debug)]
pub struct SessionInfo {
    #[serde(rename="blocklist-enabled")]
    blocklist_enabled: bool,
    #[serde(rename="download-dir")]
    download_dir: String,
    encryption: String,
    #[serde(rename="rpc-version")]
    rpc_version: i32,
    #[serde(rename="rpc-version-minimum")]
    rpc_version_minimum: i32,
    version: String,
}

pub trait RpcResponseArgument {}
impl RpcResponseArgument for SessionInfo{}