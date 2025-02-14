use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BandwidthGroup, BasicAuth, Result};
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
    let group = BandwidthGroup {
        honors_session_limits: true,
        name: "group_abc".to_string(),
        speed_limit_down_enabled: true,
        speed_limit_down: 1000,
        speed_limit_up_enabled: true,
        speed_limit_up: 2000,
    };
    let response = client.bandwidth_group_set(group).await?;
    if response.is_ok() {
        println!("Ok!");
    } else {
        println!("Err: {}", response.result);
    }
    Ok(())
}
