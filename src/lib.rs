extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;

use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;

pub mod types;

use types::BasicAuth;
use types::SessionGet;
use types::SessionStats;
use types::TorrentAction;
use types::{Id, Torrent, TorrentGetField, Torrents};
use types::{Nothing, Result, RpcRequest, RpcResponse, RpcResponseArgument};
use types::{TorrentAddArgs, TorrentAdded};

pub struct TransClient {
    url: String,
    auth: Option<BasicAuth>,
}

impl TransClient {
    /// Returns HTTP(S) client with configured Basic Auth
    pub fn with_auth(url: &str, basic_auth: BasicAuth) -> TransClient {
        TransClient {
            url: url.to_string(),
            auth: Some(basic_auth),
        }
    }

    /// Returns HTTP(S) client
    pub fn new(url: &str) -> TransClient {
        TransClient {
            url: url.to_string(),
            auth: None,
        }
    }

    /// Prepares a request for provided server and auth
    fn rpc_request(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        if let Some(auth) = &self.auth {
            client
                .post(&self.url)
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
    /// If response is impossible to unwrap then it will return an empty session_id
    async fn get_session_id(&self) -> String {
        info!("Requesting session id info");
        let response: reqwest::Result<reqwest::Response> = self
            .rpc_request()
            .json(&RpcRequest::session_get())
            .send()
            .await;
        let session_id = match response {
            Ok(ref resp) => match resp.headers().get("x-transmission-session-id") {
                Some(res) => res.to_str().expect("header value should be a string"),
                _ => "",
            },
            _ => "",
        }
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
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, SessionGet, BasicAuth};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///     let response: Result<RpcResponse<SessionGet>> = client.session_get().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!")
    ///     }
    ///     println!("Rpc reqsponse is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_get(&self) -> Result<RpcResponse<SessionGet>> {
        self.call(RpcRequest::session_get()).await
    }

    /// Performs a session stats call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, SessionStats, BasicAuth};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///     let response: Result<RpcResponse<SessionStats>> = client.session_stats().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!")
    ///     }
    ///     println!("Rpc reqsponse is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_stats(&self) -> Result<RpcResponse<SessionStats>> {
        self.call(RpcRequest::session_stats()).await
    }

    /// Performs a torrent get call
    /// fileds - if None then ALL fields
    /// ids - if None then All items
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
    /// use transmission_rpc::types::{Torrents, Torrent, TorrentGetField, Id};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///
    ///     let res: RpcResponse<Torrents<Torrent>> = client.torrent_get(None, None).await?;
    ///     let names: Vec<&String> = res.arguments.torrents.iter().map(|it| it.name.as_ref().unwrap()).collect();
    ///     println!("{:#?}", names);
    ///
    ///     let res1: RpcResponse<Torrents<Torrent>> = client.torrent_get(Some(vec![TorrentGetField::Id, TorrentGetField::Name]), Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)])).await?;
    ///     let first_three: Vec<String> = res1.arguments.torrents.iter().map(|it|
    ///         format!("{}. {}",&it.id.as_ref().unwrap(), &it.name.as_ref().unwrap())
    ///     ).collect();
    ///     println!("{:#?}", first_three);
    ///
    ///
    ///     let res2: RpcResponse<Torrents<Torrent>> = client.torrent_get(Some(vec![TorrentGetField::Id, TorrentGetField::HashString, TorrentGetField::Name]), Some(vec![Id::Hash(String::from("64b0d9a53ac9cd1002dad1e15522feddb00152fe"))])).await?;
    ///     let info: Vec<String> = res2.arguments.torrents.iter().map(|it|
    ///         format!("{:5}. {:^45} {}",
    ///             &it.id.as_ref().unwrap(),
    ///             &it.hash_string.as_ref().unwrap(),
    ///             &it.name.as_ref().unwrap())
    ///     ).collect();
    ///     println!("{:#?}", info);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_get(
        &self,
        fields: Option<Vec<TorrentGetField>>,
        ids: Option<Vec<Id>>,
    ) -> Result<RpcResponse<Torrents<Torrent>>> {
        self.call(RpcRequest::torrent_get(fields, ids)).await
    }

    /// Performs a torrent action call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
    /// use transmission_rpc::types::{TorrentAction, Nothing, Id};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///     let res1: RpcResponse<Nothing> = client.torrent_action(TorrentAction::Start, vec![Id::Id(1)]).await?;
    ///     println!("Start result: {:?}", &res1.is_ok());
    ///     let res2: RpcResponse<Nothing> = client.torrent_action(TorrentAction::Stop, vec![Id::Id(1)]).await?;
    ///     println!("Stop result: {:?}", &res2.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_action(
        &self,
        action: TorrentAction,
        ids: Vec<Id>,
    ) -> Result<RpcResponse<Nothing>> {
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
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
    /// use transmission_rpc::types::{Nothing, Id};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///     let res: RpcResponse<Nothing> = client.torrent_remove(vec![Id::Id(1)], false).await?;
    ///     println!("Remove result: {:?}", &res.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_remove(
        &self,
        ids: Vec<Id>,
        delete_local_data: bool,
    ) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::torrent_remove(ids, delete_local_data))
            .await
    }

    /// Performs a torrent set location call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
    /// use transmission_rpc::types::{Nothing, Id};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///     let res: RpcResponse<Nothing> = client.torrent_set_location(vec![Id::Id(1)], String::from("/new/location"), Option::from(false)).await?;
    ///     println!("Set-location result: {:?}", &res.is_ok());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_set_location(
        &self,
        ids: Vec<Id>,
        location: String,
        move_from: Option<bool>,
    ) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::torrent_set_location(ids, location, move_from))
            .await
    }

    /// Performs a torrent add call
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    ///
    /// # Example
    ///
    /// ```
    /// extern crate transmission_rpc;
    ///
    /// use std::env;
    /// use dotenv::dotenv;
    /// use transmission_rpc::TransClient;
    /// use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
    /// use transmission_rpc::types::{TorrentAddArgs, TorrentAdded};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url= env::var("TURL")?;
    ///     let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    ///     let client = TransClient::with_auth(&url, basic_auth);
    ///     let add: TorrentAddArgs = TorrentAddArgs {
    ///         filename: Some("https://releases.ubuntu.com/20.04/ubuntu-20.04-desktop-amd64.iso.torrent".to_string()),
    ///         ..TorrentAddArgs::default()
    ///     };
    ///     let res: RpcResponse<TorrentAdded> = client.torrent_add(add).await?;
    ///     println!("Add result: {:?}", &res.is_ok());
    ///     println!("response: {:?}", &res);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_add(&self, add: TorrentAddArgs) -> Result<RpcResponse<TorrentAdded>> {
        if add.metainfo == None && add.filename == None {
            panic!("Metainfo or Filename should be provided")
        }
        self.call(RpcRequest::torrent_add(add)).await
    }

    /// Performs a JRPC call to the server
    ///
    /// # Errors
    ///
    /// Any IO Error or Deserialization error
    async fn call<RS>(&self, request: RpcRequest) -> Result<RpcResponse<RS>>
        where
            RS: RpcResponseArgument + DeserializeOwned + std::fmt::Debug,
    {
        info!("Loaded auth: {:?}", &self.auth);
        let rq: reqwest::RequestBuilder = self
            .rpc_request()
            .header("X-Transmission-Session-Id", self.get_session_id().await)
            .json(&request);
        info!(
            "Request body: {:?}",
            rq.try_clone()
                .expect("Unable to get the request body")
                .body_string()?
        );
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
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    pub async fn test_malformed_url() -> Result<()> {
        dotenv().ok();
        env_logger::init();
        let url = env::var("TURL")?;
        let client;
        if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
            client = TransClient::with_auth(&url, BasicAuth {user, password});
        } else {
            client = TransClient::new(&url);
        }
        info!("Client is ready!");
        let add: TorrentAddArgs = TorrentAddArgs {
            filename: Some(
                "https://releases.ubuntu.com/20.04/ubuntu-20.04-desktop-amd64.iso.torrentt"
                    .to_string(),
            ),
            ..TorrentAddArgs::default()
        };
        match client.torrent_add(add).await {
            Ok(res) => {
                println!("Add result: {:?}", &res.is_ok());
                println!("response: {:?}", &res);
                assert!(!&res.is_ok());
            }
            Err(e) => {
                println!("Error: {:#?}", e);
            }
        }

        Ok(())
    }
}
