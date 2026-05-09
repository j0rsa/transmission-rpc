extern crate transmission_rpc;

use std::env;
use std::time::Duration;

use dotenvy::dotenv;
use tokio::time::sleep;
use transmission_rpc::types::{
    BasicAuth, Id, Result, RpcResponse, Torrent, TorrentGetField, TorrentGetIds, Torrents,
};
use transmission_rpc::TransClient;

/// Demonstrates `TorrentGetIds` and the `recently-active` polling pattern, a
/// method to optimize RPC bandwidth and eliminate sending redundant torrent data.
/// - First, a full fetch via torrent_get
/// - Second, a targeted fetch with TorrentGetIds
/// - Third, `recently-active` returns only torrents whose state has changes
///   since its previous call. Alongside is a list of `removed` ids the daemon
///   has deleted.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let url = env::var("TURL")?;
    let mut client = if let (Ok(user), Ok(password)) = (env::var("TUSER"), env::var("TPWD")) {
        TransClient::with_auth(url.parse()?, BasicAuth { user, password })
    } else {
        TransClient::new(url.parse()?)
    };

    let table_fields = vec![
        TorrentGetField::Id,
        TorrentGetField::HashString,
        TorrentGetField::Name,
        TorrentGetField::PercentDone,
        TorrentGetField::RateDownload,
        TorrentGetField::RateUpload,
    ];

    // 1. Seed the local view with a full fetch via torrent_get,
    //   `TorrentGetIds::Ids(vec![])` is simply expressed as `None`.
    //   After this call the daemon starts tracking which torrents
    //   change for the recently-active sentinel on subsequent requests
    //   issued over the same session.
    let seed: RpcResponse<Torrents<Torrent>> =
        client.torrent_get(Some(table_fields.clone()), None).await?;
    println!(
        "seed: {} torrents, {} removed",
        seed.arguments.torrents.len(),
        seed.arguments.removed.len()
    );

    // 2. Targeted fetch built explicitly via `TorrentGetIds::Ids`. Prefer
    // info-hash ids over numeric ids: integer ids are session-scoped and not
    // stable across Transmission daemon restarts, while the 40-character
    // info-hash uniquely identifies a torrent for its lifetime. The
    // `From<Vec<Id>>` impl makes `.into()` work too, but spelling out the
    // variant is useful when matching on it elsewhere.
    if let Some(hash) = seed
        .arguments
        .torrents
        .first()
        .and_then(|t| t.hash_string.clone())
    {
        let targeted_ids = TorrentGetIds::Ids(vec![Id::Hash(hash)]);
        if let TorrentGetIds::Ids(ref ids) = targeted_ids {
            let res: RpcResponse<Torrents<Torrent>> = client
                .torrent_get(Some(table_fields.clone()), Some(ids.clone()))
                .await?;
            for t in &res.arguments.torrents {
                println!(
                    "hash={} name={:?} done={:.1}%",
                    t.hash_string.as_deref().unwrap_or("(none)"),
                    t.name.as_deref().unwrap_or("(none)"),
                    t.percent_done.unwrap_or(0.0) * 100.0,
                );
            }
        }
    }

    // 3. Poll three times using the `recently-active` sentinel. Each tick
    // returns only the torrents that changed since the previous call plus a
    // `removed` list of ids the daemon has deleted. The first tick after the
    // seed typically reports just torrents whose stats moved (rates, eta,
    // ...) perfect for cheap UI refreshes.
    for tick in 1..=3 {
        sleep(Duration::from_secs(2)).await;
        let res: RpcResponse<Torrents<Torrent>> = client
            .torrent_get_recently_active(Some(table_fields.clone()))
            .await?;
        println!(
            "tick {tick}: {} changed, {} removed",
            res.arguments.torrents.len(),
            res.arguments.removed.len()
        );
        for t in &res.arguments.torrents {
            println!(
                "  changed hash={} ↓{}B/s ↑{}B/s",
                t.hash_string.as_deref().unwrap_or("(none)"),
                t.rate_download.unwrap_or(0),
                t.rate_upload.unwrap_or(0),
            );
        }
        // The `removed` list is protocol-defined as numeric ids only; there
        // is no hash equivalent on the wire. Map them back to hashes via a
        // local id -> hash table if you need a stable identifier.
        for id in &res.arguments.removed {
            println!("  removed id={id}");
        }
    }

    Ok(())
}
