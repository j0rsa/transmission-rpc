extern crate transmission_rpc;

use std::env;
use dotenv::dotenv;
use transmission_rpc::TransClient;
use transmission_rpc::types::{Result, RpcResponse, SessionGet, BasicAuth};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url= env::var("TURL")?;
    let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    let client = TransClient::with_auth(&url, basic_auth);
    let response: Result<RpcResponse<SessionGet>> = client.session_get().await;
    match response {
        Ok(_) => println!("Yay!"),
        Err(_) => panic!("Oh no!")
    }
    println!("Rpc reqsponse is ok: {}", response?.is_ok());
    Ok(())
}