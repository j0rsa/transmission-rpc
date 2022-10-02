extern crate transmission_rpc;

use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Id, Nothing, Result, RpcResponse};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let mut client;
    if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        client = TransClient::with_auth(&url, BasicAuth { user, password });
    } else {
        client = TransClient::new(&url);
    }
    let res: RpcResponse<Nothing> = client
        .torrent_set_location(
            vec![Id::Id(1)],
            String::from("/new/location"),
            Option::from(false),
        )
        .await?;
    println!("Set-location result: {:?}", &res.is_ok());

    Ok(())
}
