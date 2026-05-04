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
            println!("rpc-version-semver: {}", s.rpc_version_semver);
            println!("session-id: {}", s.session_id);
            println!("config-dir: {}", s.config_dir);
            println!("download-dir: {}", s.download_dir);
            println!("incomplete-dir-enabled: {}", s.incomplete_dir_enabled);
            println!("incomplete-dir: {}", s.incomplete_dir);
            println!("rename-partial-files: {}", s.rename_partial_files);
            println!(
                "trash-original-torrent-files: {}",
                s.trash_original_torrent_files
            );
            println!("start-added-torrents: {}", s.start_added_torrents);
            println!("encryption: {:?}", s.encryption);
            println!("cache-size-mb: {}", s.cache_size_mb);
            println!("default-trackers: {}", s.default_trackers);

            // Port / transport.
            println!("peer-port: {}", s.peer_port);
            println!("peer-port-random-on-start: {}", s.peer_port_random_on_start);
            println!("port-forwarding-enabled: {}", s.port_forwarding_enabled);
            println!("utp-enabled: {}", s.utp_enabled);
            println!("pex-enabled: {}", s.pex_enabled);
            println!("dht-enabled: {}", s.dht_enabled);
            println!("lpd-enabled: {}", s.lpd_enabled);
            println!("preferred-transports: {:?}", s.preferred_transports);

            // Blocklist.
            println!("blocklist-enabled: {}", s.blocklist_enabled);
            println!("blocklist-url: {}", s.blocklist_url);
            println!("blocklist-size: {}", s.blocklist_size);

            // RPC server protections.
            println!("anti-brute-force-enabled: {}", s.anti_brute_force_enabled);

            // Speed limits.
            println!("speed-limit-down-enabled: {}", s.speed_limit_down_enabled);
            println!("speed-limit-down: {}", s.speed_limit_down);
            println!("speed-limit-up-enabled: {}", s.speed_limit_up_enabled);
            println!("speed-limit-up: {}", s.speed_limit_up);

            // Alt speed (turtle mode) limits.
            println!("alt-speed-enabled: {}", s.alt_speed_enabled);
            println!("alt-speed-down: {}", s.alt_speed_down);
            println!("alt-speed-up: {}", s.alt_speed_up);
            println!("alt-speed-time-enabled: {}", s.alt_speed_time_enabled);
            println!("alt-speed-time-begin: {}", s.alt_speed_time_begin);
            println!("alt-speed-time-end: {}", s.alt_speed_time_end);
            println!("alt-speed-time-day: {}", s.alt_speed_time_day);

            // Queue / idle.
            println!("download-queue-enabled: {}", s.download_queue_enabled);
            println!("download-queue-size: {}", s.download_queue_size);
            println!("seed-queue-enabled: {}", s.seed_queue_enabled);
            println!("seed-queue-size: {}", s.seed_queue_size);
            println!("queue-stalled-enabled: {}", s.queue_stalled_enabled);
            println!("queue-stalled-minutes: {}", s.queue_stalled_minutes);
            println!(
                "idle-seeding-limit-enabled: {}",
                s.idle_seeding_limit_enabled
            );
            println!("idle-seeding-limit: {}", s.idle_seeding_limit);

            // Seed-ratio limit.
            println!("seed-ratio-limited: {}", s.seed_ratio_limited);
            println!("seed-ratio-limit: {}", s.seed_ratio_limit);

            // Peer limits / misc.
            println!("peer-limit-global: {}", s.peer_limit_global);
            println!("peer-limit-per-torrent: {}", s.peer_limit_per_torrent);
            println!("reqq: {}", s.reqq);
            println!("sequential-download: {}", s.sequential_download);

            // Scripts.
            println!(
                "script-torrent-added-enabled: {}",
                s.script_torrent_added_enabled
            );
            println!(
                "script-torrent-added-filename: {}",
                s.script_torrent_added_filename
            );
            println!(
                "script-torrent-done-enabled: {}",
                s.script_torrent_done_enabled
            );
            println!(
                "script-torrent-done-filename: {}",
                s.script_torrent_done_filename
            );
            println!(
                "script-torrent-done-seeding-enabled: {}",
                s.script_torrent_done_seeding_enabled
            );
            println!(
                "script-torrent-done-seeding-filename: {}",
                s.script_torrent_done_seeding_filename
            );
        }
        Err(_) => panic!("Oh no!"),
    }
    Ok(())
}
