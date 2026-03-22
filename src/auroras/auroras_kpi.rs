//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use chrono::Utc;
use serde_json::Value;

use super::data::Data;
use super::noaa::kpi::Kpi;
use super::noaa::scales::measurement::Measurement;
use super::util::time_tag_to_i64;

#[allow(clippy::wildcard_imports)]
use urls::*;


static KPI_FORECAST_DATA_FILE: &str = "KPI_FORECAST_DATA.json";

#[allow(dead_code)]
pub mod urls {
    // Updated every 3 hours. Also includes data in noaa-planetary-k-index.json
    pub static KPI_FORECAST_DATA_URL: &str = "https://services.swpc.noaa.gov/products/noaa-planetary-k-index-forecast.json";
    pub static KPI_FORECAST_DATA_MIRROR: &str = "https://auroras.planetpeanut.studio/noaa/products/noaa-planetary-k-index-forecast.json";
}


pub async fn auroras_kpi_get_data(data: &Data) -> Result<(), Box<dyn Error>> {
    data.download(KPI_FORECAST_DATA_URL, KPI_FORECAST_DATA_FILE).await?;
    Ok(())
}


// noaa-planetary-k-index-forecast.json
// [["time_tag","kp","observed","noaa_scale"],["2025-11-08 00:00:00","6.33","observed","G2"],
pub fn auroras_kpi(data: &Data) -> Result<Vec<Kpi>, Box<dyn Error>> {
    let mut vec = Vec::new();

    let data = data.read(KPI_FORECAST_DATA_FILE)?;
    let v: Value = serde_json::from_str(&data)?;
    let rows = v.as_array().ok_or("No data")?;

    for row in rows.iter().skip(1) {
        if let Some(array) = row.as_array() {
            let mut iter = array.iter();

            let timestamp   = iter.next().ok_or("Missing column 1")?.as_str().map(time_tag_to_i64);
            let value       = iter.next().ok_or("Missing column 2")?.as_str().and_then(|s| s.parse::<f64>().ok());
            let measurement = iter.next().ok_or("Missing column 3")?.as_str().and_then(|s| s.parse::<Measurement>().ok());

            if let (Some(timestamp), Some(value)) = (timestamp, value) {
                vec.push(
                    Kpi {
                        timestamp,
                        value,
                        measurement,
                    }
                );
            }
        }
    }

    Ok(vec)
}


pub fn auroras_kpi_current(data: &Data) -> Result<Option<Kpi>, Box<dyn Error>> {
    Ok(auroras_kpi(data)?
        .iter()
        .filter(|e| e.timestamp <= Utc::now().timestamp())
        .max_by_key(|e| e.timestamp)
        .cloned())
}


pub fn auroras_kpi_forecast(data: &Data) -> Result<Vec<Kpi>, Box<dyn Error>> {
    Ok(auroras_kpi(data)?
        .into_iter()
        .filter(|e| e.measurement == Some(Measurement::Predicted))
        .collect())
}
