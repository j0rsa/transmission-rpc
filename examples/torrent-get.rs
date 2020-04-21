extern crate transmission_rpc;

use std::env;
use dotenv::dotenv;
use transmission_rpc::TransClient;
use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
use transmission_rpc::types::{Torrents, Torrent, TorrentGetField};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url= env::var("TURL")?;
    let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    let client = TransClient::with_auth(&url, basic_auth);
    let res: RpcResponse<Torrents<Torrent>> = client.torrent_get(vec![TorrentGetField::ID, TorrentGetField::NAME]).await?;
    let names: Vec<&String> = res.arguments.torrents.iter().map(|it| it.clone().name.as_ref().unwrap()).collect();
    println!("{:#?}", names);
    Ok(())
}