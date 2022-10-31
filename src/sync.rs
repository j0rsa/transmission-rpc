use std::{ops::Deref, sync::RwLock};

use reqwest::{header::CONTENT_TYPE, Client, StatusCode, Url};
use serde::de::DeserializeOwned;

use crate::{
    types::{
        BasicAuth, BlocklistUpdate, FreeSpace, Id, Nothing, PortTest, Result, RpcRequest,
        RpcResponse, RpcResponseArgument, SessionClose, SessionGet, SessionStats, Torrent,
        TorrentAction, TorrentAddArgs, TorrentAddedOrDuplicate, TorrentGetField, TorrentRenamePath,
        TorrentSetArgs, Torrents,
    },
    BodyString, TransError, MAX_RETRIES,
};

pub struct SharableTransClient {
    url: Url,
    auth: Option<BasicAuth>,
    session_id: RwLock<Option<String>>,
    client: Client,
}

impl SharableTransClient {
    /// Returns HTTP(S) client with configured Basic Auth
    #[must_use]
    pub fn with_auth(url: Url, basic_auth: BasicAuth) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: Some(basic_auth),
            session_id: RwLock::new(None),
            client: Client::new(),
        }
    }

    /// Returns HTTP(S) client
    #[must_use]
    pub fn new(url: Url) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: None,
            session_id: RwLock::new(None),
            client: Client::new(),
        }
    }

    #[must_use]
    pub fn new_with_client(url: Url, client: Client) -> SharableTransClient {
        SharableTransClient {
            url,
            auth: None,
            session_id: RwLock::new(None),
            client,
        }
    }

    pub fn set_auth(&mut self, basic_auth: BasicAuth) {
        self.auth = Some(basic_auth);
    }

    /// Prepares a request for provided server and auth
    fn rpc_request(&self) -> reqwest::RequestBuilder {
        if let Some(auth) = &self.auth {
            self.client
                .post(self.url.clone())
                .basic_auth(&auth.user, Some(&auth.password))
        } else {
            self.client.post(self.url.clone())
        }
        .header(CONTENT_TYPE, "application/json")
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, SessionGet},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<SessionGet>> = client.session_get().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, SessionStats},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<SessionStats>> = client.session_stats().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_stats(&self) -> Result<RpcResponse<SessionStats>> {
        self.call(RpcRequest::session_stats()).await
    }

    /// Performs a session close call
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, SessionClose},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<SessionClose>> = client.session_close().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn session_close(&self) -> Result<RpcResponse<SessionClose>> {
        self.call(RpcRequest::session_close()).await
    }

    /// Performs a blocklist update call
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, BlocklistUpdate, Result, RpcResponse},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<BlocklistUpdate>> = client.blocklist_update().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn blocklist_update(&self) -> Result<RpcResponse<BlocklistUpdate>> {
        self.call(RpcRequest::blocklist_update()).await
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, FreeSpace, Result, RpcResponse},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let dir = env::var("TDIR")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<FreeSpace>> = client.free_space(dir).await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn free_space(&self, path: String) -> Result<RpcResponse<FreeSpace>> {
        self.call(RpcRequest::free_space(path)).await
    }

    /// Performs a port test call
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, PortTest, Result, RpcResponse},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let response: Result<RpcResponse<PortTest>> = client.port_test().await;
    ///     match response {
    ///         Ok(_) => println!("Yay!"),
    ///         Err(_) => panic!("Oh no!"),
    ///     }
    ///     println!("Rpc response is ok: {}", response?.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub async fn port_test(&self) -> Result<RpcResponse<PortTest>> {
        self.call(RpcRequest::port_test()).await
    }

    /// Performs a torrent get call
    /// fields - if None then ALL fields
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Result, RpcResponse, Torrent, TorrentGetField, Torrents},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///
    ///     let res: RpcResponse<Torrents<Torrent>> = client.torrent_get(None, None).await?;
    ///     let names: Vec<&String> = res
    ///         .arguments
    ///         .torrents
    ///         .iter()
    ///         .map(|it| it.name.as_ref().unwrap())
    ///         .collect();
    ///     println!("{:#?}", names);
    ///
    ///     let res1: RpcResponse<Torrents<Torrent>> = client
    ///         .torrent_get(
    ///             Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
    ///             Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
    ///         )
    ///         .await?;
    ///     let first_three: Vec<String> = res1
    ///         .arguments
    ///         .torrents
    ///         .iter()
    ///         .map(|it| {
    ///             format!(
    ///                 "{}. {}",
    ///                 &it.id.as_ref().unwrap(),
    ///                 &it.name.as_ref().unwrap()
    ///             )
    ///         })
    ///         .collect();
    ///     println!("{:#?}", first_three);
    ///
    ///     let res2: RpcResponse<Torrents<Torrent>> = client
    ///         .torrent_get(
    ///             Some(vec![
    ///                 TorrentGetField::Id,
    ///                 TorrentGetField::HashString,
    ///                 TorrentGetField::Name,
    ///             ]),
    ///             Some(vec![Id::Hash(String::from(
    ///                 "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
    ///             ))]),
    ///         )
    ///         .await?;
    ///     let info: Vec<String> = res2
    ///         .arguments
    ///         .torrents
    ///         .iter()
    ///         .map(|it| {
    ///             format!(
    ///                 "{:5}. {:^45} {}",
    ///                 &it.id.as_ref().unwrap(),
    ///                 &it.hash_string.as_ref().unwrap(),
    ///                 &it.name.as_ref().unwrap()
    ///             )
    ///         })
    ///         .collect();
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

    /// Performs a torrent set call
    /// args - the fields to update
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Result, RpcResponse, Torrent, TorrentSetArgs, Torrents},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///
    ///     let url = env::var("TURL")?.parse()?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url, basic_auth);
    ///
    ///     let args = TorrentSetArgs {
    ///         labels: Some(vec![String::from("blue")]),
    ///         ..Default::default()
    ///     };
    ///     assert!(
    ///         client
    ///             .torrent_set(args, Some(vec![Id::Id(0)]))
    ///             .await?
    ///             .is_ok()
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_set(
        &self,
        args: TorrentSetArgs,
        ids: Option<Vec<Id>>,
    ) -> Result<RpcResponse<Nothing>> {
        self.call(RpcRequest::torrent_set(args, ids)).await
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Nothing, Result, RpcResponse, TorrentAction},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res1: RpcResponse<Nothing> = client
    ///         .torrent_action(TorrentAction::Start, vec![Id::Id(1)])
    ///         .await?;
    ///     println!("Start result: {:?}", &res1.is_ok());
    ///     let res2: RpcResponse<Nothing> = client
    ///         .torrent_action(TorrentAction::Stop, vec![Id::Id(1)])
    ///         .await?;
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Nothing, Result, RpcResponse},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Nothing, Result, RpcResponse},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<Nothing> = client
    ///         .torrent_set_location(
    ///             vec![Id::Id(1)],
    ///             String::from("/new/location"),
    ///             Option::from(false),
    ///         )
    ///         .await?;
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

    /// Performs a torrent rename path call
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Id, Result, RpcResponse, TorrentRenamePath},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let res: RpcResponse<TorrentRenamePath> = client
    ///         .torrent_rename_path(
    ///             vec![Id::Id(1)],
    ///             String::from("Folder/OldFile.jpg"),
    ///             String::from("NewFile.jpg"),
    ///         )
    ///         .await?;
    ///     println!("rename-path result: {:#?}", res);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn torrent_rename_path(
        &self,
        ids: Vec<Id>,
        path: String,
        name: String,
    ) -> Result<RpcResponse<TorrentRenamePath>> {
        self.call(RpcRequest::torrent_rename_path(ids, path, name))
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
    ///
    /// use dotenvy::dotenv;
    /// use transmission_rpc::{
    ///     types::{BasicAuth, Result, RpcResponse, TorrentAddArgs, TorrentAddedOrDuplicate},
    ///     TransClient,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     dotenv().ok();
    ///     env_logger::init();
    ///     let url = env::var("TURL")?;
    ///     let basic_auth = BasicAuth {
    ///         user: env::var("TUSER")?,
    ///         password: env::var("TPWD")?,
    ///     };
    ///     let client = SharableTransClient::with_auth(url.parse()?, basic_auth);
    ///     let add: TorrentAddArgs = TorrentAddArgs {
    ///         filename: Some(
    ///             "https://releases.ubuntu.com/jammy/ubuntu-22.04.1-desktop-amd64.iso.torrent"
    ///                 .to_string(),
    ///         ),
    ///         ..TorrentAddArgs::default()
    ///     };
    ///     let res: RpcResponse<TorrentAddedOrDuplicate> = client.torrent_add(add).await?;
    ///     println!("Add result: {:?}", &res.is_ok());
    ///     println!("response: {:?}", &res);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Panics
    /// Either metainfo or torrent filename must be set or this call will panic.
    pub async fn torrent_add(
        &self,
        add: TorrentAddArgs,
    ) -> Result<RpcResponse<TorrentAddedOrDuplicate>> {
        assert!(
            !(add.metainfo.is_none() && add.filename.is_none()),
            "Metainfo or Filename should be provided"
        );
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
        let mut remaining_retries = MAX_RETRIES;
        loop {
            remaining_retries = remaining_retries
                .checked_sub(1)
                .ok_or(TransError::MaxRetriesReached)?;

            debug!("Loaded auth: {:?}", &self.auth);
            let rq = match &self.session_id.read().expect("lock being poisoned").deref() {
                None => self.rpc_request(),
                Some(id) => self.rpc_request().header("X-Transmission-Session-Id", id),
            }
            .json(&request);

            debug!(
                "Request body: {:?}",
                rq.try_clone()
                    .expect("Unable to get the request body")
                    .body_string()?
            );

            let rsp: reqwest::Response = rq.send().await?;

            info!("Response: {:?}", &rsp);
            if matches!(rsp.status(), StatusCode::CONFLICT) {
                let session_id = rsp
                    .headers()
                    .get("X-Transmission-Session-Id")
                    .ok_or(TransError::NoSessionIdReceived)?
                    .to_str()?;
                *self.session_id.write().expect("lock being poisoned") =
                    Some(String::from(session_id));

                debug!("Got new session_id: {}. Retrying request.", session_id);
            } else {
                let rpc_response: RpcResponse<RS> = rsp.json().await?;
                debug!("Response body: {:#?}", rpc_response);

                return Ok(rpc_response);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use dotenvy::dotenv;

    use super::*;

    #[tokio::test]
    pub async fn test_malformed_url() -> Result<()> {
        dotenv().ok();
        env_logger::init();
        let url = env::var("TURL")?;
        let client;
        if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
            client = SharableTransClient::with_auth(url.parse()?, BasicAuth { user, password });
        } else {
            client = SharableTransClient::new(url.parse()?);
        }
        info!("Client is ready!");
        let add: TorrentAddArgs = TorrentAddArgs {
            filename: Some(
                "https://releases.ubuntu.com/jammy/ubuntu-22.04.1-desktop-amd64.iso.torrentt"
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
