//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use chrono::{ DateTime, Utc };
use rusqlite::{ Result, params };

use crate::auroras::archive::Archive;
use crate::auroras::fediverse::tag::{ FediverseTag, TagDay };
use crate::log;


impl Archive {
    pub fn create_table_fediverse(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tag (
                timestamp INTEGER PRIMARY KEY,
                name      TEXT,
                accounts  INTEGER NOT NULL,
                uses      INTEGER NOT NULL
            ) STRICT",
            [],
        )?;

        Ok(())
    }


    pub fn insert_tag(&self, tag: &FediverseTag) -> Result<(), Box<dyn Error>> {
        for day in tag.history.iter().cloned() {
            let result = self.connection.execute(
                "INSERT INTO tag (timestamp, name, accounts, uses)
                    VALUES (?1, ?2, ?3, ?4)
                    ON CONFLICT(timestamp) DO UPDATE SET
                        accounts = excluded.accounts,
                        uses = excluded.uses
                    WHERE
                        accounts IS NOT excluded.accounts OR
                        uses     IS NOT excluded.uses;",
                params![
                    day.timestamp,
                    tag.name,
                    day.accounts,
                    day.uses,
                ]
            );

            if let Ok(count) = result {
                if count > 0 {
                    log::info(&format!("Archive | Fediverse | Inserted/updated with key {}", day.timestamp));
                    // dbg!(day);
                }
            }
        }

        Ok(())
    }


    pub fn select_day(&self, tag_name: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<TagDay>, Box<dyn Error>> {
        let mut query = self.connection.prepare(
            "SELECT timestamp, accounts, uses
             FROM tag
             WHERE timestamp BETWEEN ?1 AND ?2
               AND name = ?3
             ORDER BY timestamp ASC"
        )?;

        let rows = query.query_map(
            params![start.timestamp(), end.timestamp(), tag_name],
            |row| {
                Ok(TagDay {
                    timestamp: row.get("timestamp")?,
                    accounts: row.get("accounts")?,
                    uses: row.get("uses")?,
                })
            }
        )?;

        Ok(rows.collect::<Result<_, _>>()?)
    }
}
