extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result, RpcResponse, SessionGet};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let mut client;
    if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        client = TransClient::with_auth(url.parse()?, BasicAuth { user, password });
    } else {
        client = TransClient::new(url.parse()?);
    }
    let response: Result<RpcResponse<SessionGet>> = client.session_get().await;
    match response {
        Ok(rpc_response) => {
            println!("Rpc response is ok: {}", rpc_response.is_ok());
            let s = &rpc_response.arguments;
            println!("version: {}", s.version);
            println!("rpc-version: {}", s.rpc_version);
            println!("rpc-version-minimum: {}", s.rpc_version_minimum);
            println!("download-dir: {}", s.download_dir);
            println!("encryption: {}", s.encryption);
            println!("peer-port: {}", s.peer_port);
            println!("blocklist-enabled: {}", s.blocklist_enabled);
            println!("incomplete-dir-enabled: {:?}", s.incomplete_dir_enabled);
            println!("incomplete-dir: {:?}", s.incomplete_dir);
            println!(
                "script-torrent-done-enabled: {:?}",
                s.script_torrent_done_enabled
            );
            println!(
                "script-torrent-done-filename: {:?}",
                s.script_torrent_done_filename
            );
            println!("cache-size-mb: {:?}", s.cache_size_mb);
            println!("rename-partial-files: {:?}", s.rename_partial_files);
            println!(
                "trash-original-torrent-files: {:?}",
                s.trash_original_torrent_files
            );
            println!("start-added-torrents: {:?}", s.start_added_torrents);
        }
        Err(_) => panic!("Oh no!"),
    }
    Ok(())
}
