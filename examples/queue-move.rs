use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Id, Result};
use transmission_rpc::TransClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    env_logger::init();
    let url = env::var("TURL")?;
    let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    } else {
        TransClient::new(url.parse()?)
    };
    let response = client.queue_move_top(vec![Id::Id(1)]).await?;
    if response.is_ok() {
        println!("Ok!");
    } else {
        println!("Err: {}", response.result);
    }
    Ok(())
}
