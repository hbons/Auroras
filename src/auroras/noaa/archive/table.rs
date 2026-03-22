//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use rusqlite::Result;

use crate::auroras::archive::Archive;


impl Archive {
    pub fn create_table_imf(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS imf (
                timestamp INTEGER PRIMARY KEY,
                bx        REAL,
                by        REAL,
                bz        REAL
            ) STRICT",
            [],
        )?;

        Ok(())
    }


    pub fn create_table_kpi(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS kpi (
                timestamp   INTEGER PRIMARY KEY,
                value       REAL NOT NULL,
                measurement TEXT
            )",
            [],
        )?;

        Ok(())
    }


    pub fn create_table_oval(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS oval (
                observation_timestamp INTEGER PRIMARY KEY,
                forecast_timestamp    INTEGER NOT NULL,
                coordinates           BLOB NOT NULL
            ) STRICT",
            [],
        )?;

        Ok(())
    }


    pub fn create_table_plasma(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS plasma (
                timestamp   INTEGER PRIMARY KEY,
                density     REAL,
                speed       REAL,
                temperature REAL
            ) STRICT",
            [],
        )?;

        Ok(())
    }
}
