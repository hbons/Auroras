//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use super::data::Data;
use super::fediverse::tag::FediverseTag;

#[allow(clippy::wildcard_imports)]
use urls::*;


static FEDIVERSE_DATA_FILE: &str = "FEDIVERSE_DATA.json";

#[allow(dead_code)]
pub mod urls {
    /// Docs: https://docs.joinmastodon.org/methods/tags/#get
    pub static FEDIVERSE_DATA_URL: &str = "https://mastodon.social/api/v1/tags/aurora";
    pub static FEDIVERSE_DATA_MIRROR: &str = "https://auroras.planetpeanut.studio/mastodon/tag.json";
}


pub async fn auroras_fediverse_get_data(data: &Data) -> Result<(), Box<dyn Error>> {
    data.download(FEDIVERSE_DATA_URL, FEDIVERSE_DATA_FILE).await?;
    Ok(())
}


pub fn auroras_fediverse(data: &Data) -> Result<FediverseTag, Box<dyn Error>> {
    let data = data.read(FEDIVERSE_DATA_FILE)?;
    let tag: FediverseTag = serde_json::from_str(&data)?;

    Ok(tag)
}
