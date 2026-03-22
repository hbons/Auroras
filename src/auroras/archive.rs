//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
// use std::fs;

// use chrono::{Days, Utc};
use rusqlite::Connection;

// use crate::auroras::noaa::archive::archive_oval;

use super::data::Data;


pub struct Archive {
    pub connection: Connection,
}


impl Archive {
    pub fn from_file(data: &Data, db_name: &str) -> Result<Self, Box<dyn Error>> {
        let path = data.data_dir.join("archive").join(db_name);

        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)?;
        }

        Ok(Self {
            connection: Connection::open(&path)?
        })
    }


    // pub fn publish(&self, data: &Data) -> Result<(), Box<dyn Error>> {
    //     let path = data.data_dir.join("public");
    //     fs::create_dir_all(path)?;

    //     let start = Utc::now().checked_sub_days(Days::new(1)).ok_or("err")?;
    //     let end = Utc::now();

    //     for oval in self.select_oval(start, end)? { TODO
    //         let line = oval
    //             .coordinates()
    //             .north()
    //             .compressed();

    //         let line = oval
    //             .coordinates()
    //             .south()
    //             .compressed();
    //     }

    //     Ok(())
    // }
}
