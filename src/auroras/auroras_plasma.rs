//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use serde_json::Value;

use super::data::Data;
use super::noaa::plasma::Plasma;
use super::util::time_tag_to_i64;

#[allow(clippy::wildcard_imports)]
use urls::*;


static PLASMA_DATA_FILE: &str = "PLASMA_DATA.json";

#[allow(dead_code)]
pub mod urls {
    // Updated every 3 minutes
    pub static PLASMA_DATA_5M_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/plasma-5-minute.json";
    pub static PLASMA_DATA_2H_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/plasma-2-hour.json";
    pub static PLASMA_DATA_6H_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/plasma-6-hour.json";
    pub static PLASMA_DATA_1D_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/plasma-1-day.json";
    pub static PLASMA_DATA_3D_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/plasma-3-day.json";
    pub static PLASMA_DATA_7D_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/plasma-7-day.json";

    pub static PLASMA_DATA_MIRROR: &str = "https://auroras.planetpeanut.studio/noaa/products/solar-wind/plasma-1-day.json";
}


pub async fn auroras_plasma_get_data(data: &Data) -> Result<(), Box<dyn Error>> {
    let urls = [PLASMA_DATA_2H_URL, PLASMA_DATA_6H_URL,
        PLASMA_DATA_1D_URL, PLASMA_DATA_3D_URL, PLASMA_DATA_7D_URL ];

    data.download_with_fallbacks(&urls, PLASMA_DATA_FILE).await?;

    Ok(())
}


// [["time_tag","density","speed","temperature"],["2025-11-12 21:54:00.000","0.59","765.3","722491"],
pub fn auroras_plasma(data: &Data) -> Result<Vec<Plasma>, Box<dyn Error>> {
    let mut vec = Vec::new();

    let data = data.read(PLASMA_DATA_FILE)?;
    let v: Value = serde_json::from_str(&data)?;
    let rows = v.as_array().ok_or("No data")?;

    for row in rows.iter().skip(1).rev() {
        if let Some(array) = row.as_array() {
            let mut iter = array.iter();

            let timestamp   = iter.next().ok_or("Missing column 1")?.as_str().map(time_tag_to_i64);
            let density     = iter.next().ok_or("Missing column 2")?.as_str().and_then(|s| s.parse::<f64>().ok());
            let speed       = iter.next().ok_or("Missing column 3")?.as_str().and_then(|s| s.parse::<f64>().ok());
            let temperature = iter.next().ok_or("Missing column 4")?.as_str().and_then(|s| s.parse::<f64>().ok());

            if let Some(timestamp) = timestamp {
                vec.push(
                    Plasma {
                        timestamp,
                        density,
                        speed,
                        temperature
                    }
                );
            }
        }
    }

    Ok(vec)
}
