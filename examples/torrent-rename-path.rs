extern crate transmission_rpc;

use std::env;
use dotenv::dotenv;
use transmission_rpc::TransClient;
use transmission_rpc::types::{Result, RpcResponse, BasicAuth};
use transmission_rpc::types::{TorrentRenamePath, Id};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url= env::var("TURL")?;
    let basic_auth = BasicAuth{user: env::var("TUSER")?, password: env::var("TPWD")?};
    let client = TransClient::with_auth(&url, basic_auth);
    let res: RpcResponse<TorrentRenamePath> = client.torrent_rename_path(vec![Id::Id(1)], String::from("Folder/OldFile.jpg"), String::from("NewFile.jpg")).await?;
    println!("rename-path result: {:#?}", res);

    Ok(())
}
