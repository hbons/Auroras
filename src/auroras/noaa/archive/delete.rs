//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use chrono::{ DateTime, Utc };
use rusqlite::{ Result, params };

use crate::auroras::archive::Archive;


impl Archive {
    pub fn delete_imf(&self, before: DateTime<Utc>) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "DELETE FROM imf WHERE timestamp < ?",
            params![before.timestamp()]
        )?;

        Ok(())
    }


    pub fn delete_kpi(&self, before: DateTime<Utc>) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "DELETE FROM kpi WHERE timestamp < ?",
            params![before.timestamp()]
        )?;

        Ok(())
    }


    pub fn delete_oval(&self, before: DateTime<Utc>) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "DELETE FROM oval WHERE observation_timestamp < ?",
            params![before.timestamp()]
        )?;

        Ok(())
    }


    pub fn delete_plasma(&self, before: DateTime<Utc>) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "DELETE FROM plasma WHERE timestamp < ?",
            params![before.timestamp()]
        )?;

        Ok(())
    }
}
