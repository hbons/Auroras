//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use super::data::Data;
use super::noaa::oval::Oval;

#[allow(clippy::wildcard_imports)]
use urls::*;


static OVAL_FORECAST_DATA_FILE: &str = "OVAL_FORECAST_DATA.json";

#[allow(dead_code)]
pub mod urls {
    // Updated every 5 minutes
    pub static OVAL_FORECAST_DATA_URL: &str = "https://services.swpc.noaa.gov/json/ovation_aurora_latest.json";
    pub static OVAL_FORECAST_DATA_MIRROR: &str = "https://auroras.planetpeanut.studio/noaa/json/ovation_aurora_latest.json";
}


pub async fn auroras_oval_get_data(data: &Data) -> Result<(), Box<dyn Error>> {
    data.download(OVAL_FORECAST_DATA_URL, OVAL_FORECAST_DATA_FILE).await?;
    Ok(())
}


pub fn auroras_oval(data: &Data) -> Result<Oval, Box<dyn Error>> {
    let oval_data = data.read(OVAL_FORECAST_DATA_FILE)?;
    let forecast: Oval = serde_json::from_str(&oval_data)?;

    Ok(forecast)
}


pub fn auroras_oval_history() -> Result<(), Box<dyn Error>> {
    Ok(())
}
