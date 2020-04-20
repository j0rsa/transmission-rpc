#[allow(unused_imports)]
#[allow(dead_code)]

extern crate ajson;
extern crate bb8;
extern crate bb8_postgres;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate tokio_postgres;

use std::env;
use dotenv::dotenv;
use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
struct BasicAuth {
    user: String,
    password: String,
}

mod models;
use models::request::SessionGet;
use models::response::RpcResponse;
use models::entity::SessionInfo;

#[tokio::main]
async fn main() -> Result<()> {
    not_main().await
}

async fn not_main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let url= env::var("URL")?;
    let basic_auth = Some(BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?});
    info!("Loaded auth: {:?}", &basic_auth);
    let rq: reqwest::RequestBuilder = rpc_request(url.as_ref(), &basic_auth)
        .header("X-Transmission-Session-Id", get_session_id(url.as_ref(), &basic_auth).await)
        .json(&SessionGet::default());
    debug!("Request body: {:?}", rq.try_clone().unwrap().body_string()?);
    
    // let p2 = Point{ x: 34, ..Default::default() };

    let resp: reqwest::Response = rq.send().await?;
    // print!("{:?}", resp.text().await);
    let rpc_response: RpcResponse<SessionInfo> = resp.json().await?;
    info!("{:#?}", rpc_response);
    Ok(())
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

fn rpc_request(url: &str, basic_auth: &Option<BasicAuth>) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();
    if let Some(auth) = basic_auth {
        client.post(url.clone())
        .basic_auth(&auth.user, Some(&auth.password))
    } else {
        client.post(url.clone())
    }
    .header(CONTENT_TYPE, "application/json")
}

async fn get_session_id(url: &str, basic_auth: &Option<BasicAuth>) -> String {
    let mut map = HashMap::new();
    map.insert("method", "session-get");
    info!("Requesting session id info");
    let response: reqwest::Response = rpc_request(url, basic_auth)
    .json(&map)
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

#[cfg(test)]
mod tests {
    use crate::not_main;
    use crate::Result;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        dotenv().ok();
        not_main().await
    }
}
