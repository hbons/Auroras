//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use serde_json::Value;

use super::data::Data;
use super::noaa::imf::MagneticField;
use super::util::time_tag_to_i64;

#[allow(clippy::wildcard_imports)]
use urls::*;


static IMF_DATA_FILE: &str = "IMF_DATA.json";

#[allow(dead_code)]
pub mod urls {
    // Updated every 3 minutes
    pub static IMF_DATA_5M_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/mag-5-minute.json";
    pub static IMF_DATA_2H_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/mag-2-hour.json";
    pub static IMF_DATA_6H_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/mag-6-hour.json";
    pub static IMF_DATA_1D_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/mag-1-day.json";
    pub static IMF_DATA_3D_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/mag-3-day.json";
    pub static IMF_DATA_7D_URL: &str = "https://services.swpc.noaa.gov/products/solar-wind/mag-7-day.json";

    pub static IMF_DATA_MIRROR: &str = "https://auroras.planetpeanut.studio/noaa/products/solar-wind/mag-1-day.json";
}


pub async fn auroras_imf_get_data(data: &Data) -> Result<(), Box<dyn Error>> {
    let urls = [IMF_DATA_2H_URL, IMF_DATA_6H_URL,
        IMF_DATA_1D_URL, IMF_DATA_3D_URL, IMF_DATA_7D_URL ];

    data.download_with_fallbacks(&urls, IMF_DATA_FILE).await?;

    Ok(())
}


// [["time_tag","bx_gsm","by_gsm","bz_gsm","lon_gsm","lat_gsm","bt"],["2025-11-11 12:24:00.000","4.43","-2.13","0.41","334.29","4.81","4.94"],
pub fn auroras_imf(data: &Data) -> Result<Vec<MagneticField>, Box<dyn Error>> {
    let mut vec = Vec::new();

    let data = data.read(IMF_DATA_FILE)?;
    let v: Value = serde_json::from_str(&data)?;
    let rows = v.as_array().ok_or("No data")?;

    for row in rows.iter().skip(1).rev() {
        if let Some(array) = row.as_array() {
            let mut iter = array.iter();

            let timestamp = iter.next().ok_or("Missing column 1")?.as_str().map(time_tag_to_i64);
            let bx = iter.next().ok_or("Missing column 2")?.as_str().and_then(|s| s.parse::<f64>().ok());
            let by = iter.next().ok_or("Missing column 3")?.as_str().and_then(|s| s.parse::<f64>().ok());
            let bz = iter.next().ok_or("Missing column 4")?.as_str().and_then(|s| s.parse::<f64>().ok());

            if let Some(timestamp) = timestamp {
                vec.push(
                    MagneticField {
                        timestamp,
                        bx,
                        by,
                        bz,
                    }
                );
            }
        }
    }

    Ok(vec)
}
