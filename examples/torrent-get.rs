extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{
    BasicAuth, Id, Result, RpcResponse, Torrent, TorrentGetField, Torrents,
};
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

    let res: RpcResponse<Torrents<Torrent>> = client.torrent_get(None, None).await?;
    let names: Vec<&String> = res
        .arguments
        .torrents
        .iter()
        .map(|it| it.name.as_ref().unwrap())
        .collect();
    println!("{:#?}", names);

    let res1: RpcResponse<Torrents<Torrent>> = client
        .torrent_get(
            Some(vec![TorrentGetField::Id, TorrentGetField::Name]),
            Some(vec![Id::Id(1), Id::Id(2), Id::Id(3)]),
        )
        .await?;
    let first_three: Vec<String> = res1
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{}. {}",
                &it.id.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    println!("{:#?}", first_three);

    let res2: RpcResponse<Torrents<Torrent>> = client
        .torrent_get(
            Some(vec![
                TorrentGetField::Id,
                TorrentGetField::HashString,
                TorrentGetField::Name,
            ]),
            Some(vec![Id::Hash(String::from(
                "64b0d9a53ac9cd1002dad1e15522feddb00152fe",
            ))]),
        )
        .await?;
    let info: Vec<String> = res2
        .arguments
        .torrents
        .iter()
        .map(|it| {
            format!(
                "{:5}. {:^45} {}",
                &it.id.as_ref().unwrap(),
                &it.hash_string.as_ref().unwrap(),
                &it.name.as_ref().unwrap()
            )
        })
        .collect();
    println!("{:#?}", info);

    Ok(())
}
