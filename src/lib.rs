// #[allow(unused_imports)]
// #[allow(dead_code)]

extern crate ajson;
extern crate bb8;
extern crate bb8_postgres;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate tokio_postgres;

use serde::de::DeserializeOwned;
use reqwest::header::CONTENT_TYPE;

pub mod types;
use types::BasicAuth;
use types::{Result, RpcResponse, RpcResponseArgument, RpcRequest};
use types::SessionGet;
use types::{TorrentGetField, Torrents, Torrent};

pub struct TransClient {
    url: String,
    auth: Option<BasicAuth>
}

impl TransClient {
    pub fn with_auth(url: &str, basic_auth: BasicAuth) -> TransClient {
        TransClient {
            url: url.to_string(),
            auth: Some(basic_auth)
        }
    }

    pub fn new(url: &str) -> TransClient {
        TransClient {
            url: url.to_string(),
            auth: None
        }
    }

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

    
    pub async fn session_get(&self) -> Result<RpcResponse<SessionGet>> {
        self.call(RpcRequest::session_get()).await
    }

    pub async fn torrent_get(&self, fields: Vec<TorrentGetField>) -> Result<RpcResponse<Torrents<Torrent>>> {
        self.call(RpcRequest::torrent_get(fields)).await
    }

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

#[cfg(test)]
mod tests {
    use crate::Result;
    use crate::{TransClient, BasicAuth, TorrentGetField};
    use crate::{RpcResponse, Torrents, Torrent};
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        dotenv().ok();
        env_logger::init();
        let url= env::var("TURL")?;
        let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
        let client = TransClient::with_auth(&url, basic_auth);
        let res: RpcResponse<Torrents<Torrent>> = client.torrent_get(vec![TorrentGetField::ID, TorrentGetField::NAME]).await?;
        let names: Vec<&String> = res.arguments.torrents.iter().map(|it| it.name.as_ref().unwrap()).collect();
        println!("{:#?}", names);
        Ok(())
    }
}
