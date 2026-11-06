// SPDX-FileCopyrightText: Copyright (c) 2020 J0rsa and contributors
// SPDX-License-Identifier: MIT

use std::fmt;

use bitflags::bitflags;
use serde::{de, Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub use self::request::{
    Args, ArgumentFields, BandwidthGroupGetArgs, Method, ParseMethodError, QueueMoveArgs,
    RpcRequest, SessionSetArgs, TorrentAction, TorrentActionArgs, TorrentAddArgs, TorrentGetArgs,
    TorrentGetField, TorrentGetFormat, TorrentGetIds, TorrentRemoveArgs, TorrentRenamePathArgs,
    TorrentSetArgs, TorrentSetLocationArgs, TrackerList,
};

pub use self::response::{
    BandwidthGroups, BlocklistUpdate, ErrorType, FreeSpace, Nothing, PortTest, RpcResponse,
    RpcResponseArgument, SessionGet, SessionStats, Stats, Torrent, TorrentAddedOrDuplicate,
    TorrentRenamePath, TorrentStatus, Torrents, TrackerState,
};

/// [`Torrent`] field sub-type. You probably won't need to interact with this directly.
pub use self::response::{File, FileStat, Peer, PeersFrom, TrackerStat, Trackers};

mod request;
mod response;

#[cfg(test)]
mod tests;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Encryption {
    #[default]
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

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SchedDay : u8 {
        const Sunday    = 1 << 0;
        const Monday    = 1 << 1;
        const Tuesday   = 1 << 2;
        const Wednesday = 1 << 3;
        const Thursday  = 1 << 4;
        const Friday    = 1 << 5;
        const Saturday  = 1 << 6;

        const Weekday = Self::Monday.bits() | Self::Tuesday.bits() | Self::Wednesday.bits() | Self::Thursday.bits() | Self::Friday.bits();
        const Weekend = Self::Sunday.bits() | Self::Saturday.bits();
    }
}

impl Default for SchedDay {
    fn default() -> Self {
        Self::Weekend
    }
}

impl Serialize for SchedDay {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for SchedDay {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl de::Visitor<'_> for Visitor {
            type Value = SchedDay;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("tr_sched_day enum as number")
            }
            fn visit_i8<E>(self, v: i8) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(v as _)
            }
            fn visit_i32<E>(self, v: i32) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(v as _)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(v as _)
            }
            fn visit_u8<E>(self, v: u8) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Self::Value::from_bits_retain(v))
            }
            fn visit_u32<E>(self, v: u32) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(v as _)
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(v as _)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}
