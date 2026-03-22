//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use crate::runtime::Runtime;
use crate::auroras::archive::Archive;
use crate::auroras::auroras_fediverse;
use crate::auroras::auroras_kpi;
use crate::auroras::auroras_oval;
use crate::auroras::auroras_plasma;
use crate::auroras::auroras_imf;
use crate::auroras::data::Data;
use crate::log;


impl Runtime {
    const DEFAULT_INTERVAL: u64 = 60;

    pub async fn cli_command_mirror(&self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;

        let interval = args.get(2)
            .and_then(|s| s.strip_prefix("--interval="))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(Self::DEFAULT_INTERVAL);

        let interval = Duration::from_secs(interval);
        log::info("Mirror | Started");

        loop {
            match self.cli_command_refresh().await {
                Ok(_)  => {
                    self.cli_command_archive().await?;
                    self.cli_command_archive_fediverse().await?;
                },
                Err(e) => log::error(&format!("{e}")),
            }

            log::info(&format!("Mirror | Next check in {}s…", interval.as_secs()));
            sleep(interval);
        }
    }


    pub async fn cli_command_refresh(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;

        auroras_kpi::auroras_kpi_get_data(&data).await?;
        auroras_plasma::auroras_plasma_get_data(&data).await?;
        auroras_imf::auroras_imf_get_data(&data).await?;
        auroras_oval::auroras_oval_get_data(&data).await?;
        auroras_fediverse::auroras_fediverse_get_data(&data).await?;

        Ok(())
    }


    pub async fn cli_command_archive(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;
        let archive = Archive::from_file(&data, "noaa.db")?;

        archive.create_table_kpi()?;
        archive.create_table_plasma()?;
        archive.create_table_imf()?;
        archive.create_table_oval()?;

        for kpi in auroras_kpi::auroras_kpi(&data)? {
            archive.insert_kpi(&kpi)?;
        }

        for plasma in auroras_plasma::auroras_plasma(&data)? {
            archive.insert_plasma(&plasma)?;
        }

        for imf in auroras_imf::auroras_imf(&data)? {
            archive.insert_imf(&imf)?;
        }

        let oval = auroras_oval::auroras_oval(&data)?;
        archive.insert_oval(&oval.compress())?;

        Ok(())
    }


    pub async fn cli_command_archive_fediverse(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;

        let fedi_archive = Archive::from_file(&data, "fediverse.db")?;
        fedi_archive.create_table_fediverse()?;

        let tag = auroras_fediverse::auroras_fediverse(&data)?;
        fedi_archive.insert_tag(&tag)?;

        Ok(())
    }
}
