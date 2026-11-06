// SPDX-FileCopyrightText: Copyright (c) 2020-2026 J0rsa and contributors
// SPDX-License-Identifier: MIT

use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_with::{DeserializeAs, SerializeAs};

#[derive(Debug)]
pub struct Timestamp;

impl SerializeAs<DateTime<Utc>> for Timestamp {
    fn serialize_as<S>(source: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        chrono::serde::ts_seconds::serialize(source, serializer)
    }
}

impl<'de> DeserializeAs<'de, DateTime<Utc>> for Timestamp {
    fn deserialize_as<D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ts = i64::deserialize(deserializer)?;
        // The transmission rpc server responds with 0 or -1 (in the case of manualAnnounceTime) when
        // the date is unset or invalid.
        // Consolidate any response <= 0 as UNIX_EPOCH to denote these cases.
        if ts <= 0 {
            return Ok(DateTime::UNIX_EPOCH);
        }
        DateTime::<Utc>::from_timestamp(ts, 0)
            .ok_or(D::Error::custom("Unexpected timestamp value!"))
    }
}

#[derive(Debug)]
pub struct Bitfield;

impl SerializeAs<Vec<u8>> for Bitfield {
    fn serialize_as<S>(source: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        base64.encode(source).serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for Bitfield {
    /// Attempts to deserialize a [`base64`]-encoded string into a `Vec<u8>`.
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded: &str = Deserialize::deserialize(deserializer)?;
        base64.decode(encoded).map_err(D::Error::custom)
    }
}

#[derive(Debug)]
pub struct BoolsFromBoolsOrInts;

impl SerializeAs<Vec<bool>> for BoolsFromBoolsOrInts {
    fn serialize_as<S>(source: &Vec<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        source.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, Vec<bool>> for BoolsFromBoolsOrInts {
    /// Attempts to deserialize an array of bools or ints (`0` or `1`) into a `Vec<bool>`,
    /// treating `0` as false and `1` as true.
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = |v| match v {
            // Transmission 5.0.0+ returns an array of booleans
            Value::Bool(v) => Ok(v),
            // Transmission 4.x.x and below returns an array of ints (1 true, 0 false).
            Value::Number(n) => match n.as_i64() {
                Some(0) => Ok(false),
                Some(1) => Ok(true),
                _ => Err(D::Error::custom(format!("Unexpected int value: {n}"))),
            },
            v => Err(D::Error::custom(format!("Unexpected value: {v:?}"))),
        };
        let Value::Array(arr) = Value::deserialize(deserializer)? else {
            return Err(D::Error::custom("Unexpected non-array value!"));
        };
        arr.into_iter().map(map).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::{from_value, json};
    use serde_with::serde_as;

    #[test]
    fn test_negative_timestamp() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct Container(#[serde_as(as = "Timestamp")] DateTime<Utc>);

        let time: Container = from_value(json!(-1)).unwrap();
        assert_eq!(time.0, DateTime::UNIX_EPOCH);
    }

    #[test]
    fn test_bools_from_bools_or_ints() {
        #[serde_as]
        #[derive(Serialize, Deserialize)]
        struct Container(#[serde_as(as = "BoolsFromBoolsOrInts")] Vec<bool>);

        let bools: Container = from_value(json!([1, false, 0, true])).unwrap();
        assert_eq!(bools.0, vec![true, false, false, true]);
    }
}
