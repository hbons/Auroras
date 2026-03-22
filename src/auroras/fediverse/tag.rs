//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::str::FromStr;
use serde::{ Deserialize, Deserializer };


/// Docs: https://docs.joinmastodon.org/entities/Tag
#[derive(Clone, Debug, Deserialize)]
pub struct FediverseTag {
    #[serde(deserialize_with = "from_str")]
    pub id: u64,

    pub name: String,
    pub url: String,
    pub history: Vec<TagDay>,
}


/// Docs: https://docs.joinmastodon.org/entities/Tag/#history
#[derive(Clone, Debug, Deserialize)]
pub struct TagDay {
    #[serde(deserialize_with = "from_str", rename = "day")]
    pub timestamp: i64,

    /// Number of accounts that mentioned the tag on this day
    #[serde(deserialize_with = "from_str")]
    pub accounts: u64,

    /// Number of posts that mentioned the tag on this day
    #[serde(deserialize_with = "from_str")]
    pub uses: u64,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    D: Deserializer<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<T>().map_err(serde::de::Error::custom)
}
