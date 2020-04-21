use serde::Serialize;

#[derive(Serialize, Debug, RustcEncodable)]
pub struct RpcRequest {
    method: String,
    #[serde(skip_serializing_if="Option::is_none")]
    arguments: Option<Fields<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    ids: Option<String>,
}

impl RpcRequest {
    pub fn session_get() -> RpcRequest {
        RpcRequest { 
            method: String::from("session-get"),
            arguments: None,
            ids: None,
        }
   }

   pub fn torrent_get(fields: Vec<TorrentGetField>) -> RpcRequest {
       let string_fields = fields.iter().map(|f| f.to_str()).collect();
       RpcRequest {
        method: String::from("torrent-get"),
        arguments: Some (Fields { fields: string_fields }),
        ids: None,
       }
   }

}


pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField{}

#[derive(Serialize, Debug, RustcEncodable)]
struct Fields<T> {
    fields: Vec<T>
}

pub enum TorrentGetField {
    ID,
    ADDEDDATE,
    NAME,
    TOTALSIZE,
    ERROR,
    ERRORSTRING,
    ETA,
    ISFINISHED,
    ISSTALLED,
    LEFTUNTILDONE,
    METADATAPERCENTCOMPLETE,
    PEERSCONNECTED,
    PEERSGETTINGFROMUS,
    PEERSSENDINGTOUS,
    PERCENTDONE,
    QUEUEPOSITION,
    RATEDOWNLOAD,
    RATEUPLOAD,
    RECHECKPROGRESS,
    SEEDRATIOMODE,
    SEEDRATIOLIMIT,
    SIZEWHENDONE,
    STATUS,
    TRACKERS,
    DOWNLOADDIR,
    UPLOADEDEVER,
    UPLOADRATIO,
    WEBSEEDSSENDINGTOUS,
}

impl TorrentGetField {
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::ID => "id",
            TorrentGetField::ADDEDDATE => "addedDate",
            TorrentGetField::NAME => "name",
            TorrentGetField::TOTALSIZE => "totalSize",
            TorrentGetField::ERROR => "error",
            TorrentGetField::ERRORSTRING => "errorString",
            TorrentGetField::ETA => "eta",
            TorrentGetField::ISFINISHED => "isFinished",
            TorrentGetField::ISSTALLED => "isStalled",
            TorrentGetField::LEFTUNTILDONE => "leftUntilDone",
            TorrentGetField::METADATAPERCENTCOMPLETE => "metadataPercentComplete",
            TorrentGetField::PEERSCONNECTED => "peersConnected",
            TorrentGetField::PEERSGETTINGFROMUS => "peersGettingFromUs",
            TorrentGetField::PEERSSENDINGTOUS => "peersSendingToUs",
            TorrentGetField::PERCENTDONE => "percentDone",
            TorrentGetField::QUEUEPOSITION => "queuePosition",
            TorrentGetField::RATEDOWNLOAD => "rateDownload",
            TorrentGetField::RATEUPLOAD => "rateUpload",
            TorrentGetField::RECHECKPROGRESS => "recheckProgress",
            TorrentGetField::SEEDRATIOMODE => "seedRatioMode",
            TorrentGetField::SEEDRATIOLIMIT => "seedRatioLimit",
            TorrentGetField::SIZEWHENDONE => "sizeWhenDone",
            TorrentGetField::STATUS => "status",
            TorrentGetField::TRACKERS => "trackers",
            TorrentGetField::DOWNLOADDIR => "downloadDir",
            TorrentGetField::UPLOADEDEVER => "uploadedEver",
            TorrentGetField::UPLOADRATIO => "uploadRatio",
            TorrentGetField::WEBSEEDSSENDINGTOUS => "webseedsSendingToUs",
        }.to_string()
    }
}