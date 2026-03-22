//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use chrono::{ DateTime, Utc };
use rusqlite::{ Result, params };

use crate::auroras::archive::Archive;
use crate::auroras::noaa::imf::MagneticField;
use crate::auroras::noaa::kpi::Kpi;
use crate::auroras::noaa::oval::CompressedOval;
use crate::auroras::noaa::plasma::Plasma;


impl Archive {
    pub fn select_imf(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<MagneticField>, Box<dyn Error>> {
        let mut query = self.connection.prepare(
            "SELECT timestamp, bx, by, bz
                FROM imf
                WHERE timestamp BETWEEN ?1 AND ?2
                ORDER BY timestamp DESC"
        )?;

        let rows = query.query_map(
            params![
                start.timestamp(),
                end.timestamp()
            ],
            |row| {
                Ok(MagneticField {
                    timestamp: row.get("timestamp")?,
                    bx: row.get("bx")?,
                    by: row.get("by")?,
                    bz: row.get("bz")?,
                })
            }
        )?;

        Ok(rows.collect::<Result<_, _>>()?)
    }


    pub fn select_kpi(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Kpi>, Box<dyn Error>> {
        let mut query = self.connection.prepare(
            "SELECT timestamp, value, measurement
             FROM kpi
             WHERE timestamp BETWEEN ?1 AND ?2
             ORDER BY timestamp DESC"
        )?;

        let rows = query.query_map(
            params![
                start.timestamp(),
                end.timestamp()
            ],
            |row| {
                let measurement_str: Option<String> = row.get("measurement")?;
                let measurement = measurement_str
                    .as_deref()
                    .and_then(|s| s.parse().ok());

                Ok(Kpi {
                    timestamp: row.get("timestamp")?,
                    value: row.get("value")?,
                    measurement,
                })
            }
        )?;

        Ok(rows.collect::<Result<_, _>>()?)
    }


    pub fn select_oval(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<CompressedOval>, Box<dyn Error>> {
        let mut query = self.connection.prepare(
            "SELECT observation_timestamp, forecast_timestamp, coordinates
             FROM oval
             WHERE observation_timestamp BETWEEN ?1 AND ?2
             ORDER BY observation_timestamp DESC"
        )?;

        let rows = query.query_map(
            params![
                start.timestamp(),
                end.timestamp()
            ],
            |row| {
                let coordinates: Vec<u8> = row.get("coordinates")?;
                let coordinates = coordinates.chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();

                Ok(CompressedOval {
                    observation_time: row.get("observation_timestamp")?,
                    forecast_time: row.get("forecast_timestamp")?,
                    coordinates,
                })
            }
        )?;

        Ok(rows.collect::<Result<_, _>>()?)
    }


    pub fn select_plasma(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Plasma>, Box<dyn Error>> {
        let mut query = self.connection.prepare(
            "SELECT timestamp, density, speed, temperature
             FROM plasma
             WHERE timestamp BETWEEN ?1 AND ?2
             ORDER BY timestamp DESC"
        )?;

        let rows = query.query_map(
            params![start.timestamp(), end.timestamp()],
            |row| {
                Ok(Plasma {
                    timestamp: row.get("timestamp")?,
                    density: row.get("density")?,
                    speed: row.get("speed")?,
                    temperature: row.get("temperature")?,
                })
            }
        )?;

        Ok(rows.collect::<Result<_, _>>()?)
    }
}
