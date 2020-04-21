use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RpcResponse<T: RpcResponseArgument> {
    pub arguments: T,
    pub result: String
}

impl<T: RpcResponseArgument> RpcResponse<T> {
    pub fn is_ok(&self) -> bool {
        self.result == "success"
    }
}
pub trait RpcResponseArgument {}

#[derive(Deserialize, Debug)]
pub struct SessionGet {
    #[serde(rename="blocklist-enabled")]
    pub blocklist_enabled: bool,
    #[serde(rename="download-dir")]
    pub download_dir: String,
    pub encryption: String,
    #[serde(rename="rpc-version")]
    pub rpc_version: i32,
    #[serde(rename="rpc-version-minimum")]
    pub rpc_version_minimum: i32,
    pub version: String,
}
impl RpcResponseArgument for SessionGet{}

#[derive(Deserialize, Debug)]
pub struct Torrent {
    #[serde(rename="addedDate")]
    pub added_date: Option<i64>,
    #[serde(rename="downloadDir")]
    pub download_dir: Option<String>,
    pub error: Option<i64>,
    #[serde(rename="errorString")]
    pub error_string: Option<String>,
    pub eta: Option<i64>,
    pub id: Option<i64>,
    #[serde(rename="isFinished")]
    pub is_finished: Option<bool>,
    #[serde(rename="isStalled")]
    pub is_stalled: Option<bool>,
    #[serde(rename="leftUntilDone")]
    pub left_until_done: Option<i64>,
    #[serde(rename="metadataPercentComplete")]
    pub metadata_percent_complete: Option<f32>,
    pub name: Option<String>,
    #[serde(rename="peersConnected")]
    pub peers_connected: Option<i64>,
    #[serde(rename="peersGettingFromUs")]
    pub peers_getting_from_us: Option<i64>,
    #[serde(rename="peersSendingToUs")]
    pub peers_sending_to_us: Option<i64>,
    #[serde(rename="percentDone")]
    pub percent_done: Option<f32>,
    #[serde(rename="rateDownload")]
    pub rate_download: Option<i64>,
    #[serde(rename="rateUpload")]
    pub rate_upload: Option<i64>,
    #[serde(rename="recheckProgress")]
    pub recheck_progress: Option<f32>,
    #[serde(rename="seedRatioLimit")]
    pub seed_ratio_limit: Option<f32>,
    #[serde(rename="sizeWhenDone")]
    pub size_when_done: Option<i64>,
    pub status: Option<i64>,
    #[serde(rename="totalSize")]
    pub total_size: Option<i64>,
    #[serde(rename="uploadRatio")]
    pub upload_ratio: Option<f32>,
    #[serde(rename="uploadedEver")]
    pub uploaded_ever: Option<i64>,   
}
impl RpcResponseArgument for Torrents<Torrent>{}

#[derive(Deserialize, Debug, RustcEncodable)]
pub struct Torrents<T> {
    pub torrents: Vec<T>
}