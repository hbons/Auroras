//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use rusqlite::{ Result, params };

use crate::auroras::archive::Archive;
use crate::auroras::noaa::imf::MagneticField;
use crate::auroras::noaa::kpi::Kpi;
use crate::auroras::noaa::oval::CompressedOval;
use crate::auroras::noaa::plasma::Plasma;
use crate::log;


impl Archive {
   pub fn insert_imf(&self, imf: &MagneticField) -> Result<(), Box<dyn Error>> {
       let result = self.connection.execute(
           "INSERT INTO imf (timestamp, bx, by, bz)
               VALUES (?1, ?2, ?3, ?4)",
           params![
               imf.timestamp,
               imf.bx,
               imf.by,
               imf.bz,
           ]
       );

       if result.is_ok() {
           log::info(&format!("Archive | IMF | Inserted with key {}", imf.timestamp));
           // dbg!(imf);
       }

       Ok(())
   }


   pub fn insert_kpi(&self, kpi: &Kpi) -> Result<(), Box<dyn Error>> {
       let result = self.connection.execute(
           "INSERT INTO kpi (timestamp, value, measurement)
               VALUES (?1, ?2, ?3)",
           params![
               kpi.timestamp,
               kpi.value,
               kpi.measurement.as_ref().map(|m| m.to_short_string())
           ]
       );

       if result.is_ok() {
           log::info(&format!("Archive | KPI | Inserted with key {}", kpi.timestamp));
           // dbg!(kpi);
       }

       Ok(())
   }


   pub fn insert_oval(&self, oval: &CompressedOval) -> Result<(), Box<dyn Error>> {
       let bytes: Vec<u8> = oval.coordinates.iter()
           .flat_map(|b| b.to_le_bytes())
           .collect();

       let result = self.connection.execute(
           "INSERT INTO oval (observation_timestamp, forecast_timestamp, coordinates)
               VALUES (?1, ?2, ?3)",
           params![
               oval.observation_time,
               oval.forecast_time,
               bytes,
           ]
       );

       if result.is_ok() {
           log::info(&format!("Archive | Oval | Inserted with key {}", oval.observation_time));
           // dbg!(oval);
       }

       Ok(())
   }


   pub fn insert_plasma(&self, plasma: &Plasma) -> Result<(), Box<dyn Error>> {
       let result = self.connection.execute(
           "INSERT INTO plasma (timestamp, density, speed, temperature)
               VALUES (?1, ?2, ?3, ?4)",
           params![
               plasma.timestamp,
               plasma.density,
               plasma.speed,
               plasma.temperature,
           ]
       );

       if result.is_ok() {
           log::info(&format!("Archive | Plasma | Inserted with key {}", plasma.timestamp));
           // dbg!(plasma);
       }

       Ok(())
   }
}
