use dotenvy::dotenv;
use std::env;
use transmission_rpc::types::{BasicAuth, Result};
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
    let response = client
        .bandwidth_group_get(vec!["group_abc".to_string()].into())
        .await?;
    if response.arguments.group.is_empty() {
        println!("No bandwidth group \"group_abc\"!");
    } else {
        for g in &response.arguments.group {
            println!("Fetched \"{}\": {:?}", g.name, g);
        }
    }

    println!("-----------");
    println!("All groups:");
    let response = client.bandwidth_group_get(None).await?;
    if response.arguments.group.is_empty() {
        println!("No bandwidth groups!");
    } else {
        for g in &response.arguments.group {
            println!("{g:?}");
        }
    }
    Ok(())
}
