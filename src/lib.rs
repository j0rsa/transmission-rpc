// #[allow(unused_imports)]
// #[allow(dead_code)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;

use serde::de::DeserializeOwned;
use reqwest::header::CONTENT_TYPE;

pub mod types;
use types::BasicAuth;
use types::{Result, RpcResponse, RpcResponseArgument, RpcRequest, Nothing};
use types::SessionGet;
use types::{TorrentGetField, Torrents, Torrent};
use types::TorrentAction;
use types::{TorrentAddArgs, TorrentAdded};

pub struct TransClient {
    url: String,
    auth: Option<BasicAuth>
}

impl TransClient {
    /// Returns HTTP(S) client with configured Basic Auth
    pub fn with_auth(url: &str, basic_auth: BasicAuth) -> TransClient {
        TransClient {
            url: url.to_string(),
            auth: Some(basic_auth)
        }
    }

    /// Returns HTTP(S) client
    pub fn new(url: &str) -> TransClient {
        TransClient {
            url: url.to_string(),
            auth: None
        }
    }

    /// Prepares a request for provided server and auth
    fn rpc_request(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        if let Some(auth) = &self.auth {
            client.post(&self.url)
            .basic_auth(&auth.user, Some(&auth.password))
        } else {
            client.post(&self.url)
        }
        .header(CONTENT_TYPE, "application/json")
    }
    
    /// Performs session-get call and takes the x-transmission-session-id
    /// header to perform calls, using it's value
    /// 
    /// # Errors
    /// 
    /// Panics if any IO error happens
    async fn get_session_id(&self) -> String {
        info!("Requesting session id info");
        let response: reqwest::Response = self.rpc_request()
        .json(&RpcRequest::session_get())
        .send()
        .await
        .unwrap();
        let session_id = response.headers()
            .get("x-transmission-session-id")
            .expect("Unable to get session id")
            .to_str()
            .unwrap()
            .to_owned();
        info!("Received session id: {}", session_id);
        session_id
    }

    /// Performs a session get call
    /// 
    /// # Errors
    /// 
    /// Any IO Error or Deserialization error
    /// 
    /// # Example
    /// 
    /// in examples/session-get.rs
    pub async fn session_get(&self) -> Result<RpcResponse<SessionGet>> {
        self.call(RpcRequest::session_get()).await
    }

    /// Performs a torrent get call
    /// 
    /// # Errors
    /// 
    /// Any IO Error or Deserialization error
    /// 
    /// # Example
    /// 
    /// in examples/torrent-get.rs
    pub async fn torrent_get(&self, fields: Vec<TorrentGetField>) -> Result<RpcResponse<Torrents<Torrent>>> {
        self.call(RpcRequest::torrent_get(fields)).await
    }

    /// Performs a torrent action call
    /// 
    /// # Errors
    /// 
    /// Any IO Error or Deserialization error
    /// 
    /// # Example
    /// 
    /// in examples/torrent-action.rs
    pub async fn torrent_action(&self, action: TorrentAction, ids: Vec<i64>) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::torrent_action(action, ids)).await
    }

    /// Performs a torrent remove call
    /// 
    /// # Errors
    /// 
    /// Any IO Error or Deserialization error
    /// 
    /// # Example
    /// 
    /// in examples/torrent-remove.rs
    pub async fn torrent_remove(&self, ids: Vec<i64>, delete_local_data: bool) -> Result<RpcResponse<Nothing>> {
        self.call( RpcRequest::torrent_remove(ids, delete_local_data)).await
    }

    /// Performs a torrent add call
    /// 
    /// # Errors
    /// 
    /// Any IO Error or Deserialization error
    /// 
    /// # Example
    /// 
    /// in examples/torrent-add.rs
    pub async fn torrent_add(&self, add: TorrentAddArgs) -> Result<RpcResponse<TorrentAdded>> {
        if add.metainfo == None && add.filename == None {
            panic!("Metainfo or Filename should be provided")
        }
        self.call( RpcRequest::torrent_add(add)).await
    }

    /// Performs a JRPC call to the server
    /// 
    /// # Errors
    /// 
    /// Any IO Error or Deserialization error
    async fn call<RS> (&self, request: RpcRequest) -> Result<RpcResponse<RS>>
    where   RS : RpcResponseArgument + DeserializeOwned + std::fmt::Debug
    {
        info!("Loaded auth: {:?}", &self.auth);
        let rq: reqwest::RequestBuilder = self.rpc_request()
            .header("X-Transmission-Session-Id", self.get_session_id().await)
            .json(&request);
        info!("Request body: {:?}", rq.try_clone().unwrap().body_string()?);
        let resp: reqwest::Response = rq.send().await?;
        let rpc_response: RpcResponse<RS> = resp.json().await?;
        info!("Response body: {:#?}", rpc_response);
        Ok(rpc_response)
    }
}

trait BodyString {
    fn body_string(self) -> Result<String>;
}

impl BodyString for reqwest::RequestBuilder {
    fn body_string(self) -> Result<String> {
        let rq = self.build()?;
        let body = rq.body().unwrap().as_bytes().unwrap();
        Ok(std::str::from_utf8(body)?.to_string())
    }
}