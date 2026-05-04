// SPDX-FileCopyrightText: Copyright (c) 2020 J0rsa and contributors
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub(crate) use self::request::RpcRequest;
pub use self::request::{
    ArgumentFields, SessionSetArgs, TorrentAction, TorrentAddArgs, TorrentGetField,
    TorrentRenamePathArgs, TorrentSetArgs, TrackerList,
};

pub use self::response::{
    BandwidthGroups, BlocklistUpdate, ErrorType, FreeSpace, Nothing, PortTest, RpcResponse,
    RpcResponseArgument, SessionGet, SessionStats, Torrent, TorrentAddedOrDuplicate,
    TorrentRenamePath, TorrentStatus, Torrents, TrackerState,
};

/// [`Torrent`] field sub-type. You probably won't need to interact with this directly.
pub use self::response::{File, FileStat, Peer, PeersFrom, TrackerStat, Trackers};

mod request;
mod response;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BandwidthGroup {
    #[serde(rename = "honorsSessionLimits")]
    pub honors_session_limits: bool,
    pub name: String,
    pub speed_limit_down_enabled: bool,
    pub speed_limit_down: u64,
    pub speed_limit_up_enabled: bool,
    pub speed_limit_up: u64,
}

#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub user: String,
    pub password: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Encryption {
    Preferred,
    Required,
    Tolerated,
}

impl fmt::Display for Encryption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{self:?}").to_lowercase())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
pub enum Id {
    Id(i64),
    Hash(String),
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Priority {
    Low = -1,
    Normal = 0,
    High = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum IdleMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum RatioMode {
    Global = 0,
    Single = 1,
    Unlimited = 2,
}
