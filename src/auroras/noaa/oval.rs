//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use chrono::{ DateTime, Duration };
use serde::Deserialize;

use crate::auroras::noaa::archive::compress::{ runlength_decode, runlength_encode };


#[derive(Clone, Debug, Deserialize)]
pub struct Oval {
    /// When the aurora forecast applies
    #[serde(rename = "Observation Time", deserialize_with = "iso8601_to_unix")]
    pub observation_time: i64,

    /// When measurements were made
    #[serde(rename = "Forecast Time", deserialize_with = "iso8601_to_unix")]
    pub forecast_time: i64,

    #[serde(deserialize_with = "deserialize_coordinates")]
    pub coordinates: Vec<OvalCoordinate>,
}


#[derive(Clone, Debug, Deserialize)]
pub struct OvalCoordinate {
    /// 0 to 360°
    pub longitude: u16,

    /// –90° to +90°
    pub latitude: i8,

    /// 0 to 100
    pub probability: u8,
}


impl Oval {
    pub fn lead_time(&self) -> Duration {
        Duration::seconds(self.forecast_time - self.observation_time)
    }

    pub fn lowest_northern_latitude(&self) -> Option<OvalCoordinate> {
        self.coordinates
            .iter()
            .filter(|c| c.probability > 0)
            .filter(|c| c.latitude > 5) // Possible junk values near equator
            .min_by_key(|c| c.latitude)
            .cloned()
    }

    pub fn highest_southern_latitude(&self) -> Option<OvalCoordinate> {
        self.coordinates
            .iter()
            .filter(|c| c.probability > 0)
            .filter(|c| c.latitude < -5) // Possible junk values near equator
            .max_by_key(|c| c.latitude)
            .cloned()
    }

    pub fn highest_probability(&self) -> Option<OvalCoordinate> {
        self.coordinates
            .iter()
            .max_by_key(|c| c.probability)
            .cloned()
    }

    pub fn non_zero_count(&self) -> usize {
        self.coordinates
            .iter()
            .filter(|c| c.probability > 0)
            .count()
    }
}


impl Oval {
    /// Flattened equatorial grid of probabilities
    /// (181 rows of 360 columns, 65160 items)
    pub fn flattened(&self) -> Vec<u8> {
        let rows = 181;
        let cols = 360;
        let mut grid: Vec<Vec<u8>> = vec![vec![0; cols]; rows];

        for c in &self.coordinates {
            let col = c.longitude as usize;
            if c.latitude > 0 { // North
                let row = 90 - c.latitude as usize;
                grid[row][col] = c.probability;
            } else { // South + equator
                let row = 90 + c.latitude.unsigned_abs() as usize;
                grid[row][col] = c.probability;
            }
        }

        grid.iter()
            .flat_map(|row| row.iter())
            .cloned()
            .collect()
    }
}


impl Oval {
    pub fn compress(&self) -> CompressedOval {
        CompressedOval {
            observation_time: self.observation_time,
            forecast_time: self.forecast_time,
            coordinates: runlength_encode(&self.flattened()),
        }
    }
}


#[derive(Clone, Debug)]
pub struct CompressedOval {
    pub observation_time: i64,
    pub forecast_time: i64,

    /// Runlength-encoded bytes of the flattened
    /// coordinates (value, n, value, n, ...)
    pub coordinates: Vec<u16>,
}

impl CompressedOval {
    pub fn decompress(&self) -> Result<Oval, Box<dyn Error>> {
        let mut coordinates = vec![];

        let grid = runlength_decode(&self.coordinates)?;
        let mut iter = grid.into_iter();

        for latitude in (-90..=90).rev() {
            for longitude in 0..=359 {
                let probability = iter.next()
                    .ok_or(format!("Missing {latitude}:{longitude}"))?;

                coordinates.push(
                    OvalCoordinate {
                        longitude,
                        latitude,
                        probability,
                    }
                );
            }
        }

        coordinates.sort_by(|a, b| {
            a.longitude
                .cmp(&b.longitude)
                .then(a.latitude.cmp(&b.latitude))
        });

        Ok(Oval {
            observation_time: self.observation_time,
            forecast_time: self.observation_time,
            coordinates,
        })
    }
}


fn iso8601_to_unix<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    // "2025-11-15T17:22:00Z"
    let dt = DateTime::parse_from_rfc3339(s)
        .map_err(serde::de::Error::custom)?;

    Ok(dt.timestamp())
}


fn deserialize_coordinates<'de, D>(deserializer: D) -> Result<Vec<OvalCoordinate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: Vec<(u16, i8, u8)> = Vec::deserialize(deserializer)?;

    Ok(raw
        .into_iter()
        .map(|(longitude, latitude, value)| OvalCoordinate {
            longitude,
            latitude,
            probability: value,
        })
        .collect())
}
