use serde::Serialize;

#[derive(Serialize, Debug, RustcEncodable)]
pub struct RpcRequest {
    method: String,
    #[serde(skip_serializing_if="Option::is_none")]
    arguments: Option<Args>,
}

impl RpcRequest {
    pub fn session_get() -> RpcRequest {
        RpcRequest { 
            method: String::from("session-get"),
            arguments: None,
        }
   }

   pub fn torrent_get(fields: Vec<TorrentGetField>) -> RpcRequest {
       let string_fields = fields.iter().map(|f| f.to_str()).collect();
       RpcRequest {
        method: String::from("torrent-get"),
        arguments: Some (Args { fields: Some(string_fields), ids: None }),
       }
   }

   pub fn torrent_action(action: TorrentAction, ids: Vec<i64>) -> RpcRequest {
    RpcRequest {
        method: action.to_str(),
        arguments: Some (Args { fields: None, ids: Some(ids) }),
       }
   }
}


pub trait ArgumentFields {}
impl ArgumentFields for TorrentGetField{}

#[derive(Serialize, Debug, RustcEncodable)]
struct Args {
    #[serde(skip_serializing_if="Option::is_none")]
    fields: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    ids: Option<Vec<i64>>
}

pub enum TorrentGetField {
    Id,
    Addeddate,
    Name,
    Totalsize,
    Error,
    Errorstring,
    Eta,
    Isfinished,
    Isstalled,
    Leftuntildone,
    Metadatapercentcomplete,
    Peersconnected,
    Peersgettingfromus,
    Peerssendingtous,
    Percentdone,
    Queueposition,
    Ratedownload,
    Rateupload,
    Recheckprogress,
    Seedratiomode,
    Seedratiolimit,
    Sizewhendone,
    Status,
    Trackers,
    Downloaddir,
    Uploadedever,
    Uploadratio,
    Webseedssendingtous,
}

impl TorrentGetField {
    pub fn to_str(&self) -> String {
        match self {
            TorrentGetField::Id => "id",
            TorrentGetField::Addeddate => "addedDate",
            TorrentGetField::Name => "name",
            TorrentGetField::Totalsize => "totalSize",
            TorrentGetField::Error => "error",
            TorrentGetField::Errorstring => "errorString",
            TorrentGetField::Eta => "eta",
            TorrentGetField::Isfinished => "isFinished",
            TorrentGetField::Isstalled => "isStalled",
            TorrentGetField::Leftuntildone => "leftUntilDone",
            TorrentGetField::Metadatapercentcomplete => "metadataPercentComplete",
            TorrentGetField::Peersconnected => "peersConnected",
            TorrentGetField::Peersgettingfromus => "peersGettingFromUs",
            TorrentGetField::Peerssendingtous => "peersSendingToUs",
            TorrentGetField::Percentdone => "percentDone",
            TorrentGetField::Queueposition => "queuePosition",
            TorrentGetField::Ratedownload => "rateDownload",
            TorrentGetField::Rateupload => "rateUpload",
            TorrentGetField::Recheckprogress => "recheckProgress",
            TorrentGetField::Seedratiomode => "seedRatioMode",
            TorrentGetField::Seedratiolimit => "seedRatioLimit",
            TorrentGetField::Sizewhendone => "sizeWhenDone",
            TorrentGetField::Status => "status",
            TorrentGetField::Trackers => "trackers",
            TorrentGetField::Downloaddir => "downloadDir",
            TorrentGetField::Uploadedever => "uploadedEver",
            TorrentGetField::Uploadratio => "uploadRatio",
            TorrentGetField::Webseedssendingtous => "webseedsSendingToUs",
        }.to_string()
    }
}

pub enum TorrentAction {
    Start,
    Stop,
    StartNow,
    Verify,
    Reannounce,
}

impl TorrentAction {
    pub fn to_str(&self) -> String {
        match self {
            TorrentAction::Start => "torrent-start",
            TorrentAction::Stop => "torrent-stop",
            TorrentAction::StartNow => "torrent-start-now",
            TorrentAction::Verify => "torrent-verify",
            TorrentAction::Reannounce => "torrent-reannounce",
        }.to_string()
    }
}
