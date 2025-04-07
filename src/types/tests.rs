use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use chrono::DateTime;
use serde_json;

use crate::types::response::{TorrentStatus, TrackerState};
use crate::types::{
    ErrorType, Id, IdleMode, Priority, RatioMode, Result, RpcResponse, Torrent, Torrents,
};

type TorrentGetResp = RpcResponse<Torrents<Torrent>>;

/// torrent-get test helper to consolidate unit test boilerplate assertions.
#[allow(clippy::type_complexity)]
fn test_torrent_get(
    resp: TorrentGetResp,
    expected_len: usize,
    verify: Box<dyn Fn(&TorrentGetResp) -> Result<()>>,
) -> Result<()> {
    println!("{resp:#?}");
    assert!(resp.is_ok());
    assert_eq!(resp.arguments.torrents.len(), expected_len);
    verify(&resp)
}

const EXPECTED_MISSING_LEN: usize = 1;

fn torrent_get_only_id() -> &'static str {
    r#"
    {
        "arguments": {
            "torrents": [
                { "id":123 }
            ]
        },
        "result":"success"
    }
    "#
}

fn torrent_get_only_hash() -> &'static str {
    r#"
    {
        "arguments": {
            "torrents": [
                { "hashString":"e08c426aab2cc58649ae5e73690e3747117b3470" }
            ]
        },
        "result":"success"
    }
    "#
}

// TODO: malformed test, (?)zero length

// ----- activity_date (activityDate, ActivityDate) --------------------

#[test]
fn test_torrent_get_activity_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "activityDate":1718947434 },
                    { "activityDate":-1 },
                    { "activityDate":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].activity_date,
                Some(DateTime::parse_from_rfc3339("2024-06-21T05:23:54Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[1].activity_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].activity_date,
                Some(DateTime::UNIX_EPOCH)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_activity_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].activity_date, None);
            Ok(())
        }),
    )
}

// ----- added_date (addedDate, AddedDate) --------------------

#[test]
fn test_torrent_get_added_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "addedDate":1670612948 },
                    { "addedDate":0 },
                    { "addedDate":-1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].added_date,
                Some(DateTime::parse_from_rfc3339("2022-12-09T19:09:08Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[1].added_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].added_date,
                Some(DateTime::UNIX_EPOCH)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_added_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].added_date, None);
            Ok(())
        }),
    )
}

// ----- availability (Availability) --------------------

#[test]
#[allow(unreachable_code)] // TODO: Remove when implemented
#[ignore] // TODO: Remove when implemented
fn test_torrent_get_availability_success() -> Result<()> {
    todo!();

    let resp = serde_json::from_str(
        // TODO: Need availability data
        r#"
        {
            "arguments": {
                "torrents": [
                    { "availability":[] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].availability, Some(vec![]),);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_availability_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].availability, None);
            Ok(())
        }),
    )
}

// ----- bandwidth_priority (bandwidthPriority, BandwidthPriority) --------------------

#[test]
fn test_torrent_get_bandwidth_priority_low() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "bandwidthPriority":-1 },
                    { "bandwidthPriority":0 },
                    { "bandwidthPriority":1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].bandwidth_priority,
                Some(Priority::Low)
            );
            assert_eq!(
                resp.arguments.torrents[1].bandwidth_priority,
                Some(Priority::Normal)
            );
            assert_eq!(
                resp.arguments.torrents[2].bandwidth_priority,
                Some(Priority::High)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_bandwidth_priority_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].bandwidth_priority, None);
            Ok(())
        }),
    )
}

// ----- comment (Comment) --------------------

#[test]
fn test_torrent_get_comment_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "comment":"lorem ipsum" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].comment,
                Some("lorem ipsum".into())
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_comment_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].comment, None);
            Ok(())
        }),
    )
}

// ----- corrupt_ever (corruptEver, CorruptEver) --------------------

#[test]
fn test_torrent_get_corrupt_ever_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "corruptEver":4096 },
                    { "corruptEver":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].corrupt_ever, Some(4096));
            assert_eq!(resp.arguments.torrents[1].corrupt_ever, Some(0));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_corrupt_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].corrupt_ever, None);
            Ok(())
        }),
    )
}

// ----- creator (Creator) --------------------

#[test]
fn test_torrent_get_creator_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "creator":"mktorrent 1.1" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].creator,
                Some("mktorrent 1.1".into())
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_creator_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].creator, None);
            Ok(())
        }),
    )
}

// ----- date_created (dateCreated, DateCreated) --------------------

#[test]
fn test_torrent_get_date_created_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "dateCreated":1592962706 },
                    { "dateCreated":0 },
                    { "dateCreated":-1 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].date_created,
                Some(DateTime::parse_from_rfc3339("2020-06-24T01:38:26Z")?.to_utc()),
            );
            assert_eq!(
                resp.arguments.torrents[1].date_created,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].date_created,
                Some(DateTime::UNIX_EPOCH)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_date_created_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].date_created, None);
            Ok(())
        }),
    )
}

// ----- desired_available (desiredAvailable, DesiredAvailable) --------------------

#[test]
fn test_torrent_get_desired_available_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "desiredAvailable":20162576 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].desired_available, Some(20162576));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_desired_available_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].desired_available, None);
            Ok(())
        }),
    )
}

// ----- done_date (doneDate, DoneDate) --------------------

#[test]
fn test_torrent_get_done_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "doneDate":0 },
                    { "doneDate":-1 },
                    { "doneDate":1672060369 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].done_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[1].done_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].done_date,
                Some(DateTime::parse_from_rfc3339("2022-12-26T13:12:49Z")?.to_utc()),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_done_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].done_date, None);
            Ok(())
        }),
    )
}

// ----- download_dir (downloadDir, DownloadDir) --------------------

#[test]
fn test_torrent_get_download_dir_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadDir":"/downloads/iso/" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].download_dir,
                Some("/downloads/iso/".into())
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_download_dir_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].download_dir, None);
            Ok(())
        }),
    )
}

// ----- downloaded_ever (downloadedEver, DownloadedEver) --------------------

#[test]
fn test_torrent_get_downloaded_ever_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadedEver":0 },
                    { "downloadedEver":1340189370 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].downloaded_ever, Some(0));
            assert_eq!(resp.arguments.torrents[1].downloaded_ever, Some(1340189370));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_downloaded_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].downloaded_ever, None);
            Ok(())
        }),
    )
}

// ----- download_limit (downloadLimit, DownloadLimit) --------------------

#[test]
fn test_torrent_get_download_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadLimit":0 },
                    { "downloadLimit":1024 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].download_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].download_limit, Some(1024));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_download_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].download_limit, None);
            Ok(())
        }),
    )
}

// ----- download_limited (downloadLimited, DownloadLimited) --------------------

#[test]
fn test_torrent_get_download_limited_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "downloadLimited":true },
                    { "downloadLimited":false }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].download_limited, Some(true));
            assert_eq!(resp.arguments.torrents[1].download_limited, Some(false));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_download_limited_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].download_limited, None);
            Ok(())
        }),
    )
}

// ----- edit_date (editDate, DownloadLimited) --------------------

#[test]
fn test_torrent_get_edit_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "editDate":0 },
                    { "editDate":-1 },
                    { "editDate":1723512675 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].edit_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[1].edit_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].edit_date,
                Some(DateTime::parse_from_rfc3339("2024-08-13T01:31:15Z")?.to_utc()),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_edit_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].edit_date, None);
            Ok(())
        }),
    )
}

// ----- error (Error) --------------------

#[test]
fn test_torrent_get_error_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "error":0 },
                    { "error":1 },
                    { "error":2 },
                    { "error":3 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        4,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].error, Some(ErrorType::Ok));
            assert_eq!(
                resp.arguments.torrents[1].error,
                Some(ErrorType::TrackerWarning)
            );
            assert_eq!(
                resp.arguments.torrents[2].error,
                Some(ErrorType::TrackerError)
            );
            assert_eq!(
                resp.arguments.torrents[3].error,
                Some(ErrorType::LocalError)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_error_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].error, None);
            Ok(())
        }),
    )
}

// ----- error_string (errorString, ErrorString) --------------------

#[test]
fn test_torrent_get_error_string_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "errorString":"" },
                    { "errorString":"Unregistered torrent" },
                    { "errorString":"No data found! Ensure your drives are connected or use \"Set Location\". To re-download, remove the torrent and re-add it." }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].error_string, Some("".into()));
            assert_eq!(
                resp.arguments.torrents[1].error_string,
                Some("Unregistered torrent".into())
            );
            assert_eq!(
                resp.arguments.torrents[2].error_string,
                Some(
                    "No data found! Ensure your drives are connected or use \"Set Location\". \
                To re-download, remove the torrent and re-add it."
                        .into()
                ),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_error_string_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].error_string, None);
            Ok(())
        }),
    )
}

// ----- eta (Eta) --------------------

#[test]
fn test_torrent_get_eta_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "eta":-1 },
                    { "eta":82112 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].eta, Some(-1));
            assert_eq!(resp.arguments.torrents[1].eta, Some(82112));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_eta_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].eta, None);
            Ok(())
        }),
    )
}

// ----- eta_idle (etaIdle, EtaIdle) --------------------

#[test]
fn test_torrent_get_eta_idle_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "etaIdle":-1 },
                    { "etaIdle":1234 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].eta_idle, Some(-1));
            assert_eq!(resp.arguments.torrents[1].eta_idle, Some(1234));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_eta_idle_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].eta_idle, None);
            Ok(())
        }),
    )
}

// ----- file_count (file-count, FileCount) --------------------

#[test]
fn test_torrent_get_file_count_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "file-count":0 },
                    { "file-count":31 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].file_count, Some(0)); // Probably impossible
            assert_eq!(resp.arguments.torrents[1].file_count, Some(31));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_file_count_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].file_count, None);
            Ok(())
        }),
    )
}

// ----- files (Files) --------------------

/// Pre- rpc-version `18` (transmission `4.1.0`) test where the `File`s will contain neither
/// `begin_piece` nor `end_piece`.
#[test]
fn test_torrent_get_files_success_pre_rpc_ver_18() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "files":[{
                        "bytesCompleted":172415250,
                        "length":3994091520,
                        "name":"debian-12.6.0-amd64-DVD-1.iso"
                    }] },
                    { "files":[
                        {
                            "bytesCompleted":0,
                            "length":1229,
                            "name":"Fedora-Server-40-1.14-x86_64-CHECKSUM"
                        },
                        {
                            "bytesCompleted":0,
                            "length":2612854784,
                            "name":"Fedora-Server-dvd-x86_64-40-1.14.iso"
                        }
                    ] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            let first = resp.arguments.torrents[0]
                .files
                .as_ref()
                .expect("files should exist");
            assert_eq!(first.len(), 1);
            assert_eq!(first[0].length, 3994091520);
            assert_eq!(first[0].bytes_completed, 172415250);
            assert_eq!(first[0].name, "debian-12.6.0-amd64-DVD-1.iso");
            assert_eq!(first[0].begin_piece, None);
            assert_eq!(first[0].end_piece, None);
            let second = resp.arguments.torrents[1]
                .files
                .as_ref()
                .expect("files should exist");
            assert_eq!(second.len(), 2);
            assert_eq!(second[0].length, 1229);
            assert_eq!(second[0].bytes_completed, 0);
            assert_eq!(second[0].name, "Fedora-Server-40-1.14-x86_64-CHECKSUM");
            assert_eq!(second[0].begin_piece, None);
            assert_eq!(second[0].end_piece, None);
            assert_eq!(second[1].length, 2612854784);
            assert_eq!(second[1].bytes_completed, 0);
            assert_eq!(second[1].name, "Fedora-Server-dvd-x86_64-40-1.14.iso");
            assert_eq!(second[1].begin_piece, None);
            assert_eq!(second[1].end_piece, None);
            Ok(())
        }),
    )
}

/// Post- rpc-version `18` (transmission `4.1.0`) test where the `File`s **will** contain both
/// `begin_piece` and `end_piece`.
#[test]
fn test_torrent_get_files_success_post_rpc_ver_18() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "files":[
                        {
                            "bytesCompleted":0,
                            "length":1229,
                            "name":"Fedora-Server-40-1.14-x86_64-CHECKSUM",
                            "beginPiece": 0,
                            "endPiece": 123456
                        },
                        {
                            "bytesCompleted":0,
                            "length":2612854784,
                            "name":"Fedora-Server-dvd-x86_64-40-1.14.iso",
                            "beginPiece": 123456,
                            "endPiece": 234567
                        }
                    ] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            let files = resp.arguments.torrents[0]
                .files
                .as_ref()
                .expect("files should exist");
            assert_eq!(files.len(), 2);
            assert_eq!(files[0].length, 1229);
            assert_eq!(files[0].bytes_completed, 0);
            assert_eq!(files[0].name, "Fedora-Server-40-1.14-x86_64-CHECKSUM");
            assert_eq!(files[0].begin_piece, Some(0));
            assert_eq!(files[0].end_piece, Some(123456));

            assert_eq!(files[1].length, 2612854784);
            assert_eq!(files[1].bytes_completed, 0);
            assert_eq!(files[1].name, "Fedora-Server-dvd-x86_64-40-1.14.iso");
            assert_eq!(files[1].begin_piece, Some(123456));
            assert_eq!(files[1].end_piece, Some(234567));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_files_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].files.is_none());
            Ok(())
        }),
    )
}

// ----- file_stats (fileStats, FileStats) --------------------

#[test]
fn test_torrent_get_file_stats_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { 
                        "fileStats":[
                            {
                                "bytesCompleted": 2972771,
                                "priority": 0,
                                "wanted": true
                            },
                            {
                                "bytesCompleted": 17662350,
                                "priority": 1,
                                "wanted": true
                            },
                            {
                                "bytesCompleted": 0,
                                "priority": -1,
                                "wanted": false
                            }
                        ]
                    },
                    { 
                        "fileStats":[
                            {
                                "bytesCompleted": 0,
                                "priority": 1,
                                "wanted": false
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            let first = resp.arguments.torrents[0]
                .file_stats
                .as_ref()
                .expect("file_stats should exist");
            assert_eq!(first.len(), 3);
            assert_eq!(first[0].bytes_completed, 2972771);
            assert_eq!(first[0].priority, Priority::Normal);
            assert!(first[0].wanted);
            assert_eq!(first[1].bytes_completed, 17662350);
            assert_eq!(first[1].priority, Priority::High);
            assert!(first[1].wanted);
            assert_eq!(first[2].bytes_completed, 0);
            assert_eq!(first[2].priority, Priority::Low);
            assert!(!first[2].wanted);
            let second = resp.arguments.torrents[1]
                .file_stats
                .as_ref()
                .expect("file_stats should exist");
            assert_eq!(second[0].bytes_completed, 0);
            assert_eq!(second[0].priority, Priority::High);
            assert!(!second[0].wanted);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_file_stats_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].file_stats.is_none());
            Ok(())
        }),
    )
}

// ----- group (Group) --------------------

#[test]
fn test_torrent_get_group_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "group":"foo" } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].group, Some("foo".into()));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_group_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].group, None);
            Ok(())
        }),
    )
}

// ----- hash_string (hashString, HashString) --------------------

#[test]
fn test_torrent_get_hash_string_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "hashString":"7fce8abbdacefd47321700ff95106447009aa1e7" } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].hash_string,
                Some("7fce8abbdacefd47321700ff95106447009aa1e7".into()),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_hash_string_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].hash_string, None);
            Ok(())
        }),
    )
}

// ----- have_unchecked (haveUnchecked, HaveUnchecked) --------------------

#[test]
fn test_torrent_get_have_unchecked_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "haveUnchecked":39813 },
                    { "haveUnchecked":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].have_unchecked, Some(39813));
            assert_eq!(resp.arguments.torrents[1].have_unchecked, Some(0));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_have_unchecked_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].have_unchecked, None);
            Ok(())
        }),
    )
}

// ----- have_valid (haveValid, HaveValid) --------------------

#[test]
fn test_torrent_get_have_valid_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "haveValid":1276581443 },
                    { "haveValid":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].have_valid, Some(1276581443));
            assert_eq!(resp.arguments.torrents[1].have_valid, Some(0));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_have_valid_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].have_valid, None);
            Ok(())
        }),
    )
}

// ----- honors_session_limits (honorsSessionLimits, HonorsSessionLimits) --------------------

#[test]
fn test_torrent_get_honors_session_limits_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "honorsSessionLimits":false },
                    { "honorsSessionLimits":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].honors_session_limits,
                Some(false)
            );
            assert_eq!(resp.arguments.torrents[1].honors_session_limits, Some(true));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_honors_session_limits_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].honors_session_limits, None);
            Ok(())
        }),
    )
}

// ----- id (Id) --------------------

#[test]
fn test_torrent_get_id_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "id":111 } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].id, Some(111));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_id_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_hash())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].id, None);
            Ok(())
        }),
    )
}

// ----- is_finished (isFinished, IsFinished) --------------------

#[test]
fn test_torrent_get_is_finished_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "isFinished":true },
                    { "isFinished":false }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].is_finished, Some(true));
            assert_eq!(resp.arguments.torrents[1].is_finished, Some(false));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_is_finished_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].is_finished, None);
            Ok(())
        }),
    )
}

// ----- is_private (isPrivate, IsPrivate) --------------------

#[test]
fn test_torrent_get_is_private_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "isPrivate":false },
                    { "isPrivate":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].is_private, Some(false));
            assert_eq!(resp.arguments.torrents[1].is_private, Some(true));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_is_private_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].is_private, None);
            Ok(())
        }),
    )
}

// ----- is_stalled (isStalled, IsStalled) --------------------

#[test]
fn test_torrent_get_is_stalled_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "isStalled":false },
                    { "isStalled":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].is_stalled, Some(false));
            assert_eq!(resp.arguments.torrents[1].is_stalled, Some(true));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_is_stalled_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].is_stalled, None);
            Ok(())
        }),
    )
}

// ----- labels (Labels) --------------------

#[test]
fn test_torrent_get_labels_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "labels":[] },
                    { "labels":["foo"] },
                    { "labels":["bar","baz"] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].labels, Some(vec![]));
            assert_eq!(resp.arguments.torrents[1].labels, Some(vec!["foo".into()]));
            assert_eq!(
                resp.arguments.torrents[2].labels,
                Some(vec!["bar".into(), "baz".into()])
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_labels_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].labels, None);
            Ok(())
        }),
    )
}

// ----- left_until_done (leftUntilDone, LeftUntilDone) --------------------

#[test]
fn test_torrent_get_left_until_done_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "leftUntilDone":2138956824 },
                    { "leftUntilDone":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].left_until_done, Some(2138956824));
            assert_eq!(resp.arguments.torrents[1].left_until_done, Some(0));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_left_until_done_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].left_until_done, None);
            Ok(())
        }),
    )
}

// ----- magnet_link (magnetLink, MagnetLink) --------------------

#[test]
fn test_torrent_get_magnet_link_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "magnetLink":"" },
                    { "magnetLink":"magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810&dn=archlinux-2024.08.01-x86_64.iso" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].magnet_link, Some("".into()));
            assert_eq!(
                resp.arguments.torrents[1].magnet_link,
                Some(
                    "magnet:?xt=urn:btih:cfc214278888c26cb1516399a304c4f74ff6a810\
                &dn=archlinux-2024.08.01-x86_64.iso"
                        .into()
                ),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_magnet_link_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].magnet_link, None);
            Ok(())
        }),
    )
}

// ----- manual_announce_time (manualAnnounceTime, ManualAnnounceTime) --------------------

#[test]
fn test_torrent_get_manual_announce_time_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "manualAnnounceTime":-1 },
                    { "manualAnnounceTime":0 },
                    { "manualAnnounceTime":1723512975 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].manual_announce_time,
                Some(DateTime::UNIX_EPOCH),
            );
            assert_eq!(
                resp.arguments.torrents[1].manual_announce_time,
                Some(DateTime::UNIX_EPOCH),
            ); // Might be impossible
            assert_eq!(
                resp.arguments.torrents[2].manual_announce_time,
                Some(DateTime::parse_from_rfc3339("2024-08-13T01:36:15Z")?.to_utc()),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_manual_announce_time_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].manual_announce_time, None);
            Ok(())
        }),
    )
}

// ----- max_connected_peers (maxConnectedPeers, MaxConnectedPeers) --------------------

#[test]
fn test_torrent_get_max_connected_peers_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "maxConnectedPeers":0 },
                    { "maxConnectedPeers":20 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].max_connected_peers, Some(0));
            assert_eq!(resp.arguments.torrents[1].max_connected_peers, Some(20));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_max_connected_peers_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].max_connected_peers, None);
            Ok(())
        }),
    )
}

// ----- metadata_percent_complete (metadataPercentComplete, MetadataPercentComplete) ----------

#[test]
fn test_torrent_get_metadata_percent_complete_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "metadataPercentComplete":1 },
                    { "metadataPercentComplete":0 },
                    { "metadataPercentComplete":0.5284 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].metadata_percent_complete,
                Some(1.)
            );
            assert_eq!(
                resp.arguments.torrents[1].metadata_percent_complete,
                Some(0.)
            );
            assert_eq!(
                resp.arguments.torrents[2].metadata_percent_complete,
                Some(0.5284)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_metadata_percent_complete_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].metadata_percent_complete, None);
            Ok(())
        }),
    )
}

// ----- name (Name) --------------------

#[test]
fn test_torrent_get_name_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ { "name":"debian-12.6.0-amd64-DVD-1.iso" } ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].name,
                Some("debian-12.6.0-amd64-DVD-1.iso".into())
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_name_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].name, None);
            Ok(())
        }),
    )
}

// ----- peer_limit (peer-limit, PeerLimit) --------------------

#[test]
fn test_torrent_get_peer_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peer-limit":0 },
                    { "peer-limit":20 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peer_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].peer_limit, Some(20));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_peer_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peer_limit, None);
            Ok(())
        }),
    )
}

// ----- peers (Peers) --------------------

#[test]
fn test_torrent_get_peers_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peers":[] },
                    {
                        "peers":[
                            {
                                "address":"10.0.0.100",
                                "clientName":"\u00b5Torrent 3.5.5",
                                "clientIsChoked":false,
                                "clientIsInterested":true,
                                "flagStr":"dUEI",
                                "isDownloadingFrom":false,
                                "isEncrypted":true,
                                "isIncoming":true,
                                "isUploadingTo":true,
                                "isUTP":false,
                                "peerIsChoked":false,
                                "peerIsInterested":true,
                                "port":55555,
                                "progress":0.2641,
                                "rateToClient":0,
                                "rateToPeer":385000
                            },
                            {
                                "address":"2001:0db8:85a3:0000:0000:8a2e:0370:7334",
                                "clientName":"qBittorrent 4.6.5",
                                "clientIsChoked":false,
                                "clientIsInterested":true,
                                "flagStr":"TDI",
                                "isDownloadingFrom":true,
                                "isEncrypted":false,
                                "isIncoming":true,
                                "isUploadingTo":false,
                                "isUTP":true,
                                "peerIsChoked":true,
                                "peerIsInterested":false,
                                "port":36667,
                                "progress":1,
                                "rateToClient":8000,
                                "rateToPeer":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            let first = resp.arguments.torrents[0]
                .peers
                .as_ref()
                .expect("peers should exist");
            assert_eq!(first.len(), 0);
            let second = resp.arguments.torrents[1]
                .peers
                .as_ref()
                .expect("peers should exist");
            assert_eq!(second.len(), 2);
            assert_eq!(second[0].address, IpAddr::V4(Ipv4Addr::new(10, 0, 0, 100)));
            assert_eq!(second[0].client_name, "µTorrent 3.5.5".to_string());
            assert!(!second[0].client_is_choked);
            assert!(second[0].client_is_interested);
            assert_eq!(second[0].flag_str, "dUEI".to_string());
            assert!(!second[0].is_downloading_from);
            assert!(second[0].is_encrypted);
            assert!(second[0].is_incoming);
            assert!(second[0].is_uploading_to);
            assert!(!second[0].is_utp);
            assert!(!second[0].peer_is_choked);
            assert!(second[0].peer_is_interested);
            assert_eq!(second[0].port, 55555);
            assert_eq!(second[0].progress, 0.2641);
            assert_eq!(second[0].rate_to_client, 0);
            assert_eq!(second[0].rate_to_peer, 385000);
            assert_eq!(
                second[1].address,
                IpAddr::V6(Ipv6Addr::new(8193, 3512, 34211, 0, 0, 35374, 880, 29492))
            );
            assert_eq!(second[1].client_name, "qBittorrent 4.6.5".to_string());
            assert!(!second[1].client_is_choked);
            assert!(second[1].client_is_interested);
            assert_eq!(second[1].flag_str, "TDI".to_string());
            assert!(second[1].is_downloading_from);
            assert!(!second[1].is_encrypted);
            assert!(second[1].is_incoming);
            assert!(!second[1].is_uploading_to);
            assert!(second[1].is_utp);
            assert!(second[1].peer_is_choked);
            assert!(!second[1].peer_is_interested);
            assert_eq!(second[1].port, 36667);
            assert_eq!(second[1].progress, 1.);
            assert_eq!(second[1].rate_to_client, 8000);
            assert_eq!(second[1].rate_to_peer, 0);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_peers_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].peers.is_none());
            Ok(())
        }),
    )
}

// ----- peers_connected (peersConnected, PeersConnected) --------------------

#[test]
fn test_torrent_get_peers_connected_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peersConnected":0 },
                    { "peersConnected":6 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peers_connected, Some(0));
            assert_eq!(resp.arguments.torrents[1].peers_connected, Some(6));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_peers_connected_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peers_connected, None);
            Ok(())
        }),
    )
}

// ----- peers_from (peersFrom, PeersFrom) --------------------

#[test]
fn test_torrent_get_peers_from_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { 
                        "peersFrom": {
                            "fromCache":0,
                            "fromDht":1,
                            "fromIncoming":2,
                            "fromLpd":3,
                            "fromLtep":4,
                            "fromPex":5,
                            "fromTracker":6
                        }
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            let peers = resp.arguments.torrents[0]
                .peers_from
                .as_ref()
                .expect("peers_from should exist");
            assert_eq!(peers.from_cache, 0);
            assert_eq!(peers.from_dht, 1);
            assert_eq!(peers.from_incoming, 2);
            assert_eq!(peers.from_lpd, 3);
            assert_eq!(peers.from_ltep, 4);
            assert_eq!(peers.from_pex, 5);
            assert_eq!(peers.from_tracker, 6);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_peers_from_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].peers_from.is_none());
            Ok(())
        }),
    )
}

// ----- peers_getting_from_us (peersGettingFromUs, PeersGettingFromUs) --------------------

#[test]
fn test_torrent_get_peers_getting_from_us_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peersGettingFromUs":0 },
                    { "peersGettingFromUs":2 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peers_getting_from_us, Some(0));
            assert_eq!(resp.arguments.torrents[1].peers_getting_from_us, Some(2));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_peers_getting_from_us_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peers_getting_from_us, None);
            Ok(())
        }),
    )
}

// ----- peers_sending_to_us (peersSendingToUs, PeersSendingToUs) --------------------

#[test]
fn test_torrent_get_peers_sending_to_us_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "peersSendingToUs":0 },
                    { "peersSendingToUs":9 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peers_sending_to_us, Some(0));
            assert_eq!(resp.arguments.torrents[1].peers_sending_to_us, Some(9));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_peers_sending_to_us_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].peers_sending_to_us, None);
            Ok(())
        }),
    )
}

// ----- percent_complete (percentComplete, PercentComplete) --------------------

#[test]
fn test_torrent_get_percent_complete_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "percentComplete":1 },
                    { "percentComplete":0 },
                    { "percentComplete":0.321 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].percent_complete, Some(1.));
            assert_eq!(resp.arguments.torrents[1].percent_complete, Some(0.));
            assert_eq!(resp.arguments.torrents[2].percent_complete, Some(0.321));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_percent_complete_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].percent_complete, None);
            Ok(())
        }),
    )
}

// ----- percent_done (percentDone, PercentDone) --------------------

#[test]
fn test_torrent_get_percent_done_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "percentDone":0 },
                    { "percentDone":1 },
                    { "percentDone":0.4231 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].percent_done, Some(0.));
            assert_eq!(resp.arguments.torrents[1].percent_done, Some(1.));
            assert_eq!(resp.arguments.torrents[2].percent_done, Some(0.4231));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_percent_done_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].percent_done, None);
            Ok(())
        }),
    )
}

// ----- pieces (Pieces) --------------------

#[test]
fn test_torrent_get_pieces_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 

                    { "pieces":"/Pb49/m+8tPzi+Z/e/39" },

                    { "pieces":"//////////////////////////////////////////////////////////////////////////////////////////////////////////////////w=" },

                    { "pieces":"AAAAAAAAAAAA" }

                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            let first = resp.arguments.torrents[0]
                .pieces
                .as_ref()
                .expect("pieces should exist");
            assert_eq!(first.len(), 15); // 120 pieces (8 * 15 = 120)
            let bitfield: Vec<u8> = vec![
                0xFC, 0xF6, 0xF8, 0xF7, 0xF9, 0xBE, 0xF2, 0xD3, 0xF3, 0x8B, 0xE6, 0x7F, 0x7B, 0xFD,
                0xFD,
            ];
            assert_eq!(first, &bitfield);

            let second = resp.arguments.torrents[1]
                .pieces
                .as_ref()
                .expect("pieces should exist");
            assert_eq!(second.len(), 86); // 686 pieces (8 * 86 = 688 => 2 extra bits)
            let mut bitfield = vec![u8::MAX; 85];
            bitfield.push(0xFC);
            assert_eq!(second, &bitfield);

            let third = resp.arguments.torrents[2]
                .pieces
                .as_ref()
                .expect("pieces should exist");
            assert_eq!(third.len(), 9); // 72 pieces (8 * 9 = 72)
            let bitfield = vec![0u8; 9];
            assert_eq!(third, &bitfield);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_pieces_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].pieces.is_none());
            Ok(())
        }),
    )
}

// ----- piece_count (pieceCount, PieceCount) --------------------

#[test]
fn test_torrent_get_piece_count_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "pieceCount":10234 },
                    { "pieceCount":9876 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].piece_count, Some(10234));
            assert_eq!(resp.arguments.torrents[1].piece_count, Some(9876));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_piece_count_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].piece_count, None);
            Ok(())
        }),
    )
}

// ----- piece_size (pieceSize, PieceSize) --------------------

#[test]
fn test_torrent_get_piece_size_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "pieceSize":2097152 },
                    { "pieceSize":1048576 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].piece_size, Some(2097152));
            assert_eq!(resp.arguments.torrents[1].piece_size, Some(1048576));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_piece_size_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].piece_size, None);
            Ok(())
        }),
    )
}

// ----- primary_mime_type (primary-mime-type, PrimaryMimeType) --------------------

#[test]
fn test_torrent_get_primary_mime_type_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "primary-mime-type":"application/octet-stream" },
                    { "primary-mime-type":"audio/x-flac" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].primary_mime_type,
                Some("application/octet-stream".into()),
            );
            assert_eq!(
                resp.arguments.torrents[1].primary_mime_type,
                Some("audio/x-flac".into())
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_primary_mime_type_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].primary_mime_type, None);
            Ok(())
        }),
    )
}

// ----- queue_position (queuePosition, QueuePosition) --------------------

#[test]
fn test_torrent_get_queue_position_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "queuePosition":0 },
                    { "queuePosition":1 },
                    { "queuePosition":342 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].queue_position, Some(0));
            assert_eq!(resp.arguments.torrents[1].queue_position, Some(1));
            assert_eq!(resp.arguments.torrents[2].queue_position, Some(342));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_queue_position_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].queue_position, None);
            Ok(())
        }),
    )
}

// ----- rate_download (rateDownload, RateDownload) --------------------

#[test]
fn test_torrent_get_rate_download_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "rateDownload":93000 },
                    { "rateDownload":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].rate_download, Some(93000));
            assert_eq!(resp.arguments.torrents[1].rate_download, Some(0));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_rate_download_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].rate_download, None);
            Ok(())
        }),
    )
}

// ----- rate_upload (rateUpload, RateUpload) --------------------

#[test]
fn test_torrent_get_rate_upload_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "rateUpload":0 },
                    { "rateUpload":150000 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].rate_upload, Some(0));
            assert_eq!(resp.arguments.torrents[1].rate_upload, Some(150000));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_rate_upload_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].rate_upload, None);
            Ok(())
        }),
    )
}

// ----- recheck_progress (recheckProgress, RecheckProgress) --------------------

#[test]
fn test_torrent_get_recheck_progress_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "recheckProgress":0 },
                    { "recheckProgress":1 },
                    { "recheckProgress":0.4051 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].recheck_progress, Some(0.));
            assert_eq!(resp.arguments.torrents[1].recheck_progress, Some(1.));
            assert_eq!(resp.arguments.torrents[2].recheck_progress, Some(0.4051));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_recheck_progress_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].recheck_progress, None);
            Ok(())
        }),
    )
}

// ----- seconds_downloading (secondsDownloading, SecondsDownloading) --------------------

#[test]
fn test_torrent_get_seconds_downloading_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "secondsDownloading":0 },
                    { "secondsDownloading":41744 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seconds_downloading, Some(0));
            assert_eq!(resp.arguments.torrents[1].seconds_downloading, Some(41744));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_seconds_downloading_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seconds_downloading, None);
            Ok(())
        }),
    )
}

// ----- seconds_seeding (secondsSeeding, SecondsSeeding) --------------------

#[test]
fn test_torrent_get_seconds_seeding_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "secondsSeeding":0 },
                    { "secondsSeeding":13359445 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seconds_seeding, Some(0));
            assert_eq!(resp.arguments.torrents[1].seconds_seeding, Some(13359445));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_seconds_seeding_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seconds_seeding, None);
            Ok(())
        }),
    )
}

// ----- seed_idle_limit (seedIdleLimit, SeedIdleLimit) --------------------

#[test]
fn test_torrent_get_seed_idle_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [ 
                    { "seedIdleLimit":0 },
                    { "seedIdleLimit":30 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seed_idle_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].seed_idle_limit, Some(30));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_seed_idle_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seed_idle_limit, None);
            Ok(())
        }),
    )
}

// ----- seed_idle_mode (seedIdleMode, SeedIdleMode) --------------------

#[test]
fn test_torrent_get_seed_idle_mode_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "seedIdleMode":0 },
                    { "seedIdleMode":1 },
                    { "seedIdleMode":2 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].seed_idle_mode,
                Some(IdleMode::Global)
            );
            assert_eq!(
                resp.arguments.torrents[1].seed_idle_mode,
                Some(IdleMode::Single)
            );
            assert_eq!(
                resp.arguments.torrents[2].seed_idle_mode,
                Some(IdleMode::Unlimited)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_seed_idle_mode_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seed_idle_mode, None);
            Ok(())
        }),
    )
}

// ----- seed_ratio_limit (seedRatioLimit, SeedRatioLimit) --------------------

#[test]
fn test_torrent_get_seed_ratio_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "seedRatioLimit":0 },
                    { "seedRatioLimit":0.25 },
                    { "seedRatioLimit":15 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seed_ratio_limit, Some(0.));
            assert_eq!(resp.arguments.torrents[1].seed_ratio_limit, Some(0.25));
            assert_eq!(resp.arguments.torrents[2].seed_ratio_limit, Some(15.));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_seed_ratio_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seed_ratio_limit, None);
            Ok(())
        }),
    )
}

// ----- seed_ratio_mode (seedRatioMode, SeedRatioMode) --------------------

#[test]
fn test_torrent_get_seed_ratio_mode_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "seedRatioMode":2 },
                    { "seedRatioMode":1 },
                    { "seedRatioMode":0 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].seed_ratio_mode,
                Some(RatioMode::Unlimited)
            );
            assert_eq!(
                resp.arguments.torrents[1].seed_ratio_mode,
                Some(RatioMode::Single)
            );
            assert_eq!(
                resp.arguments.torrents[2].seed_ratio_mode,
                Some(RatioMode::Global)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_seed_ratio_mode_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].seed_ratio_mode, None);
            Ok(())
        }),
    )
}

// ----- sequential_download (sequentialDownload, SequentialDownload) --------------------

#[test]
fn test_torrent_get_sequential_download_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "sequentialDownload":true },
                    { "sequentialDownload":false }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].sequential_download, Some(true));
            assert_eq!(resp.arguments.torrents[1].sequential_download, Some(false));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_sequential_download_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].sequential_download, None);
            Ok(())
        }),
    )
}

// ----- size_when_done (sizeWhenDone, SizeWhenDone) --------------------

#[test]
fn test_torrent_get_size_when_done_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "sizeWhenDone":2965366874 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].size_when_done, Some(2965366874));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_size_when_done_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].size_when_done, None);
            Ok(())
        }),
    )
}

// ----- start_date (startDate, StartDate) --------------------

#[test]
fn test_torrent_get_start_date_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "startDate":0 },
                    { "startDate":-1 },
                    { "startDate":1723479770 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].start_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[1].start_date,
                Some(DateTime::UNIX_EPOCH)
            );
            assert_eq!(
                resp.arguments.torrents[2].start_date,
                Some(DateTime::parse_from_rfc3339("2024-08-12T16:22:50Z")?.to_utc()),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_start_date_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].start_date, None);
            Ok(())
        }),
    )
}

// ----- status (Status) --------------------

#[test]
fn test_torrent_get_status_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "status":0 },
                    { "status":1 },
                    { "status":2 },
                    { "status":3 },
                    { "status":4 },
                    { "status":5 },
                    { "status":6 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        7,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].status,
                Some(TorrentStatus::Stopped)
            );
            assert_eq!(
                resp.arguments.torrents[1].status,
                Some(TorrentStatus::QueuedToVerify)
            );
            assert_eq!(
                resp.arguments.torrents[2].status,
                Some(TorrentStatus::Verifying)
            );
            assert_eq!(
                resp.arguments.torrents[3].status,
                Some(TorrentStatus::QueuedToDownload)
            );
            assert_eq!(
                resp.arguments.torrents[4].status,
                Some(TorrentStatus::Downloading)
            );
            assert_eq!(
                resp.arguments.torrents[5].status,
                Some(TorrentStatus::QueuedToSeed)
            );
            assert_eq!(
                resp.arguments.torrents[6].status,
                Some(TorrentStatus::Seeding)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_status_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].status, None);
            Ok(())
        }),
    )
}

// ----- torrent_file (torrentFile, TorrentFile) --------------------

#[test]
fn test_torrent_get_torrent_file_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "torrentFile":"/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(
                resp.arguments.torrents[0].torrent_file,
                Some("/torrents/36119b75587513a6b577df2a3747f7ae3e152394.torrent".into()),
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_torrent_file_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].torrent_file, None);
            Ok(())
        }),
    )
}

// ----- total_size (totalSize, TotalSize) --------------------

#[test]
fn test_torrent_get_total_size_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "totalSize":2050306968 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].total_size, Some(2050306968));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_total_size_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].total_size, None);
            Ok(())
        }),
    )
}

// ----- trackers (Trackers) --------------------

#[test]
fn test_torrent_get_trackers_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    {
                        "trackers": [
                            {
                                "announce":"https://example.com:1024/announce",
                                "id":0,
                                "scrape":"https://example.com:1024/scrape",
                                "tier":0
                            },
                            {
                                "announce":"https://example.com:2048/announce",
                                "id":1,
                                "scrape":"https://example.com:2048/scrape",
                                "tier":0
                            }
                        ]
                    },
                    {
                        "trackers": [
                            {
                                "announce":"https://example.com:4096/announce",
                                "id":0,
                                "scrape":"https://example.com:4096/scrape",
                                "sitename":"example",
                                "tier":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            let first = resp.arguments.torrents[0]
                .trackers
                .as_ref()
                .expect("trackers should exist");
            assert_eq!(first.len(), 2);
            assert_eq!(first[0].id, 0);
            assert_eq!(first[0].announce, "https://example.com:1024/announce");
            assert_eq!(first[0].scrape, "https://example.com:1024/scrape");
            assert_eq!(first[0].sitename, "");
            assert_eq!(first[0].tier, 0);
            assert_eq!(first[1].id, 1);
            assert_eq!(first[1].announce, "https://example.com:2048/announce");
            assert_eq!(first[1].scrape, "https://example.com:2048/scrape");
            assert_eq!(first[1].sitename, "");
            assert_eq!(first[1].tier, 0);
            let second = resp.arguments.torrents[1]
                .trackers
                .as_ref()
                .expect("trackers should exist");
            assert_eq!(second.len(), 1);
            assert_eq!(second[0].id, 0);
            assert_eq!(second[0].announce, "https://example.com:4096/announce");
            assert_eq!(second[0].scrape, "https://example.com:4096/scrape");
            assert_eq!(second[0].sitename, "example");
            assert_eq!(second[0].tier, 0);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_trackers_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].trackers.is_none());
            Ok(())
        }),
    )
}

// ----- tracker_list (trackerList, TrackerList) --------------------

#[test]
#[allow(unreachable_code)] // TODO: Remove when implemented
#[ignore] // TODO: Remove when implemented
fn test_torrent_get_tracker_list_success() -> Result<()> {
    todo!();

    let resp = serde_json::from_str(
        // TODO: Need trackerList data
        r#"
        {
            "arguments": {
                "torrents": [
                    { "trackerList":"" }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].tracker_list, Some("".into()));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_tracker_list_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].tracker_list, None);
            Ok(())
        }),
    )
}

// ----- tracker_stats (trackerStats, TrackerStats) --------------------

#[test]
fn test_torrent_get_tracker_stats_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    {
                        "trackerStats": [
                            {
                                "announce":"https://example.com/announce",
                                "announceState":1,
                                "downloadCount":245,
                                "hasAnnounced":true,
                                "hasScraped":true,
                                "host":"https://example.com:8080",
                                "id":0,
                                "isBackup":false,
                                "lastAnnouncePeerCount":86,
                                "lastAnnounceResult":"Success",
                                "lastAnnounceStartTime":1723614865,
                                "lastAnnounceSucceeded":true,
                                "lastAnnounceTime":1723614865,
                                "lastAnnounceTimedOut":false,
                                "lastScrapeResult":"Could not connect to tracker",
                                "lastScrapeStartTime":0,
                                "lastScrapeSucceeded":false,
                                "lastScrapeTime":1723614865,
                                "lastScrapeTimedOut":false,
                                "leecherCount":9,
                                "nextAnnounceTime":1723618230,
                                "nextScrapeTime":0,
                                "scrapeState":2,
                                "scrape":"",
                                "seederCount":77,
                                "tier":0
                            }
                        ]
                    }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            let first = resp.arguments.torrents[0]
                .tracker_stats
                .as_ref()
                .expect("tracker_stats should exist");
            assert_eq!(first.len(), 1);
            assert!(matches!(first[0].announce_state, TrackerState::Waiting));
            assert_eq!(
                first[0].announce,
                "https://example.com/announce".to_string()
            );
            assert_eq!(first[0].download_count, 245);
            assert!(first[0].has_announced);
            assert!(first[0].has_scraped);
            assert_eq!(first[0].host, "https://example.com:8080");
            assert!(matches!(first[0].id, Id::Id(0)));
            assert!(!first[0].is_backup);
            assert_eq!(first[0].last_announce_peer_count, 86);
            assert_eq!(first[0].last_announce_result, "Success".to_string());
            assert_eq!(
                first[0].last_announce_start_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert!(first[0].last_announce_succeeded);
            assert_eq!(
                first[0].last_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert!(!first[0].last_announce_timed_out);
            assert_eq!(
                first[0].last_scrape_result,
                "Could not connect to tracker".to_string()
            );
            assert_eq!(first[0].last_scrape_start_time, DateTime::UNIX_EPOCH);
            assert!(!first[0].last_scrape_succeeded);
            assert_eq!(
                first[0].last_scrape_time,
                DateTime::parse_from_rfc3339("2024-08-14T05:54:25Z")?.to_utc(),
            );
            assert!(!first[0].last_scrape_timed_out);
            assert_eq!(first[0].leecher_count, 9);
            assert_eq!(
                first[0].next_announce_time,
                DateTime::parse_from_rfc3339("2024-08-14T06:50:30Z")?.to_utc(),
            );
            assert_eq!(first[0].next_scrape_time, DateTime::UNIX_EPOCH);
            assert!(matches!(first[0].scrape_state, TrackerState::Queued));
            assert_eq!(first[0].scrape, "".to_string());
            assert_eq!(first[0].seeder_count, 77);
            assert_eq!(first[0].sitename, "".to_string());
            assert_eq!(first[0].tier, 0);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_tracker_stats_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert!(resp.arguments.torrents[0].tracker_stats.is_none());
            Ok(())
        }),
    )
}

// ----- upload_ratio (uploadRatio, UploadRatio) --------------------

#[test]
fn test_torrent_get_upload_ratio_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadRatio":-1 },
                    { "uploadRatio":0 },
                    { "uploadRatio":1.23 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        3,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].upload_ratio, Some(-1.));
            assert_eq!(resp.arguments.torrents[1].upload_ratio, Some(0.));
            assert_eq!(resp.arguments.torrents[2].upload_ratio, Some(1.23));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_upload_ratio_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].upload_ratio, None);
            Ok(())
        }),
    )
}

// ----- uploaded_ever (uploadedEver, UploadedEver) --------------------

#[test]
fn test_torrent_get_uploaded_ever_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadedEver":0 },
                    { "uploadedEver":1301396208 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].uploaded_ever, Some(0));
            assert_eq!(resp.arguments.torrents[1].uploaded_ever, Some(1301396208));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_uploaded_ever_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].uploaded_ever, None);
            Ok(())
        }),
    )
}

// ----- upload_limit (uploadLimit, UploadLimit) --------------------

#[test]
fn test_torrent_get_uploaded_limit_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadLimit":0 },
                    { "uploadLimit":1024 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].upload_limit, Some(0));
            assert_eq!(resp.arguments.torrents[1].upload_limit, Some(1024));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_uploaded_limit_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].upload_limit, None);
            Ok(())
        }),
    )
}

// ----- upload_limited (uploadLimited, UploadLimited) --------------------

#[test]
fn test_torrent_get_uploaded_limited_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "uploadLimited":false },
                    { "uploadLimited":true }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].upload_limited, Some(false));
            assert_eq!(resp.arguments.torrents[1].upload_limited, Some(true));
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_uploaded_limited_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].upload_limited, None);
            Ok(())
        }),
    )
}

// ----- wanted (Wanted) --------------------

#[test]
fn test_torrent_get_wanted_int_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "wanted":[0, 1, 0, 0, 1] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            let wanted = resp.arguments.torrents[0]
                .wanted
                .as_ref()
                .expect("wanted is some");
            assert_eq!(wanted, &vec![false, true, false, false, true]);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_wanted_bool_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "wanted":[true, true, false, false, true, false, true] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        1,
        Box::new(|resp: &TorrentGetResp| {
            let wanted = resp.arguments.torrents[0]
                .wanted
                .as_ref()
                .expect("wanted is some");
            assert_eq!(wanted, &vec![true, true, false, false, true, false, true]);
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_wanted_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].wanted, None);
            Ok(())
        }),
    )
}

// ----- webseeds (Webseeds) --------------------

#[test]
fn test_torrent_get_webseeds_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "webseeds":[] },
                    { "webseeds":["https://example.com/"] }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].webseeds, Some(vec![]));
            assert_eq!(
                resp.arguments.torrents[1].webseeds,
                Some(vec!["https://example.com/".into()])
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_webseeds_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].webseeds, None);
            Ok(())
        }),
    )
}

// ----- webseeds_sending_to_us (webseedsSendingToUs, WebseedsSendingToUs) --------------------

#[test]
fn test_torrent_get_webseeds_sending_to_us_success() -> Result<()> {
    let resp = serde_json::from_str(
        r#"
        {
            "arguments": {
                "torrents": [
                    { "webseedsSendingToUs":0 },
                    { "webseedsSendingToUs":1234 }
                ]
            },
            "result":"success"
        }
        "#,
    )?;
    test_torrent_get(
        resp,
        2,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].webseeds_sending_to_us, Some(0));
            assert_eq!(
                resp.arguments.torrents[1].webseeds_sending_to_us,
                Some(1234)
            );
            Ok(())
        }),
    )
}

#[test]
fn test_torrent_get_webseeds_sending_to_us_missing() -> Result<()> {
    let resp = serde_json::from_str(torrent_get_only_id())?;
    test_torrent_get(
        resp,
        EXPECTED_MISSING_LEN,
        Box::new(|resp: &TorrentGetResp| {
            assert_eq!(resp.arguments.torrents[0].webseeds_sending_to_us, None);
            Ok(())
        }),
    )
}
