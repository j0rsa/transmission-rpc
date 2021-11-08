extern crate transmission_rpc;

use dotenv::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse, FreeSpace};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let dir = env::var("TDIR")?;
    let client;
    if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        client = TransClient::with_auth(&url, BasicAuth {user, password});
    } else {
        client = TransClient::new(&url);
    }
    let response: Result<RpcResponse<FreeSpace>> = client.free_space(dir).await;
    match response {
        Ok(_) => println!("Yay!"),
        Err(_) => panic!("Oh no!")
    }
    println!("Rpc response is ok: {}", response?.is_ok());
    Ok(())
}
