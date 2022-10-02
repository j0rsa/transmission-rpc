extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Id, Result, RpcResponse, TorrentRenamePath};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let basic_auth = BasicAuth {
        user: env::var("TUSER")?,
        password: env::var("TPWD")?,
    };
    let mut client = TransClient::with_auth(&url, basic_auth);
    let res: RpcResponse<TorrentRenamePath> = client
        .torrent_rename_path(
            vec![Id::Id(1)],
            String::from("Folder/OldFile.jpg"),
            String::from("NewFile.jpg"),
        )
        .await?;
    println!("rename-path result: {:#?}", res);

    Ok(())
}
