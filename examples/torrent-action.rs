extern crate transmission_rpc;

use std::env;
use dotenv::dotenv;
use transmission_rpc::TransClient;
use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
use transmission_rpc::types::{TorrentAction, Nothing, Id};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url= env::var("TURL")?;
    let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    let client = TransClient::with_auth(&url, basic_auth);
    let res1: RpcResponse<Nothing> = client.torrent_action(TorrentAction::Start, vec![Id::Id(1)]).await?;
    println!("Start result: {:?}", &res1.is_ok());
    let res2: RpcResponse<Nothing> = client.torrent_action(TorrentAction::Stop, vec![Id::Id(1)]).await?;
    println!("Stop result: {:?}", &res2.is_ok());
    
    Ok(())
}