extern crate transmission_rpc;

use dotenv::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse};
use transmission_rpc::types::{TorrentAddArgs, TorrentAdded};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let client;
    if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        client = TransClient::with_auth(&url, BasicAuth {user, password});
    } else {
        client = TransClient::new(&url);
    }
    let add: TorrentAddArgs = TorrentAddArgs {
        filename: Some("https://releases.ubuntu.com/20.04/ubuntu-20.04.2.0-desktop-amd64.iso.torrent".to_string()),
        ..TorrentAddArgs::default()
    };
    let res: RpcResponse<TorrentAdded> = client.torrent_add(add).await?;
    println!("Add result: {:?}", &res.is_ok());
    println!("response: {:?}", &res);

    Ok(())
}
