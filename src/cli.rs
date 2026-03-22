//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use chrono::DateTime;
use reqwest::Url;

use crate::runtime::{app_version, Runtime};
use crate::auroras::auroras_fediverse;
use crate::auroras::auroras_imf;
use crate::auroras::auroras_kpi;
use crate::auroras::auroras_oval;
use crate::auroras::auroras_plasma;
use crate::auroras::data::Data;
use crate::auroras::noaa::scales::geomagnetic::GeomagneticActivityScale;

impl Runtime {
    pub async fn cli_parse_args(&mut self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        self.cli_require_args(1, args)?;
        let command = args.get(1).ok_or("Missing <command>")?;

        if Data::from_directory(&self.app_cache_home).is_err() {
            self.cli_command_refresh().await?;
        }

        match command.as_str() {
            "archive" => self.cli_command_archive().await?,
            "fediverse" => self.cli_command_fediverse()?,
            "kpi" => self.cli_command_kpi()?,
            "mirror" => self.cli_command_mirror(args).await?,
            "oval" => self.cli_command_oval()?,
            "solarwind" => self.cli_command_solarwind()?,
            "refresh" => self.cli_command_refresh().await?,
            "--help" => self.cli_option_help(),
            "--version" => println!("{}", app_version()),
            "--env" => println!("{:#?}", self),
            _ => {
                self.cli_option_help();
                return Err("Unknown command".into());
            }
        }

        Ok(())
    }

    pub fn cli_option_help(&self) {
        println!("Usage: auroras <command>");
        println!();
        println!("Commands:");
        println!("    fediverse");
        println!("    kpi");
        println!("    solarwind");
        println!("    oval");
        println!();
        println!("Server Commands:");
        println!("    archive");
        println!("    mirror [--interval=60]");
        println!("    refresh");
        println!();
        println!("Sources:");
        println!("    NOAA’s Space Weather Prediction Center (NOAA/SWPC)");
        // println!("    Norwegian Meteorological Institute (MET Norway)");
        println!();
        println!("Options:");
        println!("    --help, --version, --env");
        println!();
    }
}

impl Runtime {
    pub fn cli_command_fediverse(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;

        cli_title(0, "Fediverse");
        println!();

        if let Ok(tag) = auroras_fediverse::auroras_fediverse(&data) {
            if let Ok(url) = Url::parse(&tag.url) {
                cli_prop(
                    4,
                    "Tag",
                    &format!("#{}", &cli_link(&tag.url, Some(&tag.name))),
                    &format!("on {}", url.domain().ok_or("Missing domain")?),
                );
            }

            println!();
            cli_title(2, "Posts per day");

            for day in tag.history {
                cli_prop(4, &cli_time(day.timestamp), &day.uses.to_string(), "");
            }

            println!();
        }

        Ok(())
    }

    pub fn cli_command_kpi(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;

        cli_title(0, "Planetary K-index");
        println!();

        if let Some(kpi) = auroras_kpi::auroras_kpi_current(&data)? {
            cli_prop(2, "Current", &kpi.to_string(), "");
            cli_prop(
                2,
                "Geomagnetic Activity",
                &GeomagneticActivityScale::from_kpi(&kpi).to_string(),
                "",
            );
            cli_prop(2, "Timestamp", &cli_time(kpi.timestamp), "UTC");
            println!();
        }

        cli_title(2, "Forecast");

        for kpi in auroras_kpi::auroras_kpi_forecast(&data)? {
            cli_prop(4, &cli_time(kpi.timestamp), &kpi.to_string(), "");
        }

        println!();

        Ok(())
    }

    pub fn cli_command_oval(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;

        cli_title(0, "Aurora Oval");
        println!();

        if let Ok(oval) = auroras_oval::auroras_oval(&data) {
            cli_title(2, "Forecast");
            cli_prop(4, "Forecast Time", &cli_time(oval.forecast_time), "UTC");
            cli_prop(
                4,
                "Observation Time",
                &cli_time(oval.observation_time),
                "UTC",
            );
            cli_prop(
                4,
                "Lead Time",
                &oval.lead_time().num_minutes().to_string(),
                "min",
            );
            println!();

            cli_title(2, "Coordinates");
            cli_prop(
                4,
                "Non-Zero Coordinates",
                &((oval.non_zero_count() as f64) / (oval.coordinates.len() as f64) * 100.0)
                    .round()
                    .to_string(),
                "%",
            );

            if let Some(c) = oval.highest_probability() {
                cli_prop(4, "Highest Probability", &c.probability.to_string(), "%");
            }

            if let Some(c) = oval.lowest_northern_latitude() {
                cli_prop(4, "Lowest Northern Latitude", &c.latitude.to_string(), "°");
            }

            if let Some(c) = oval.highest_southern_latitude() {
                cli_prop(4, "Highest Southern Latitude", &c.latitude.to_string(), "°");
            }

            println!();
        }

        Ok(())
    }

    pub fn cli_command_solarwind(&self) -> Result<(), Box<dyn Error>> {
        let data = Data::from_directory(&self.app_cache_home)?;

        println!("{}", cli_bold("Solar Wind"));
        println!();

        let plasma_current = auroras_plasma::auroras_plasma(&data)?
            .into_iter()
            .max_by_key(|e| e.timestamp);

        if let Some(plasma) = plasma_current {
            cli_title(2, "Plasma");
            cli_prop(4, "Speed", &plasma.speed.unwrap_or(0.0).to_string(), "km/h");
            cli_prop(
                4,
                "Density",
                &plasma.density.unwrap_or(0.0).to_string(),
                "cm⁻³",
            );
            cli_prop(
                4,
                "Temperature",
                &plasma.temperature.unwrap_or(0.0).to_string(),
                "K",
            );
            cli_prop(4, "Timestamp", &cli_time(plasma.timestamp), "UTC");
            println!();
        }

        let imf_latest = auroras_imf::auroras_imf(&data)?
            .into_iter()
            .max_by_key(|e| e.timestamp);

        if let Some(imf) = imf_latest {
            cli_title(2, "Interplanetary Magnetic Field");
            cli_prop(4, "Bx", &imf.bx.unwrap_or(0.0).to_string(), "nT");
            cli_prop(4, "By", &imf.by.unwrap_or(0.0).to_string(), "nT");
            cli_prop(4, "Bz", &imf.bz.unwrap_or(0.0).to_string(), "nT");
            cli_prop(
                4,
                "Longitude",
                &format!("{:.2}", &imf.longitude().unwrap_or(0.0)),
                "°",
            );
            cli_prop(
                4,
                "Latitude",
                &format!("{:.2}", &imf.latitude().unwrap_or(0.0)),
                "°",
            );
            cli_prop(4, "Bt", &format!("{:.2}", &imf.bt().unwrap_or(0.0)), "nT");
            cli_prop(4, "Timestamp", &cli_time(imf.timestamp), "UTC");
            println!();
        }

        Ok(())
    }
}

impl Runtime {
    /// Checks if the minimum amount of args have been passed
    pub(crate) fn cli_require_args(
        &self,
        count: usize,
        args: &[String],
    ) -> Result<(), Box<dyn Error>> {
        if args.len() - 1 < count {
            self.cli_option_help();
            return Err(format!("Command requires {count} arguments").into());
        }

        Ok(())
    }
}


pub fn cli_indent(indent: usize, value: &str) {
    println!("{}{}", " ".repeat(indent), value);
}

pub fn cli_title(indent: usize, value: &str) {
    cli_indent(indent, &cli_bold(value));
}

pub fn cli_time(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "—".to_string())
}

pub fn cli_prop(indent: usize, label: &str, value: &str, unit: &str) {
    let display_unit = if unit == "°" {
        unit
    } else {
        &format!(" {}", unit)
    };

    println!(
        "{}{}:  {}{}",
        " ".repeat(indent),
        label,
        cli_bold(value),
        cli_dimmed(display_unit)
    );
}

// Docs: https://jvns.ca/blog/2025/03/07/escape-code-standards/
pub fn cli_bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
pub fn cli_dimmed(s: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", s)
}
pub fn cli_error(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn cli_link(url: &str, label: Option<&str>) -> String {
    format!(
        "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        url,
        label.unwrap_or(url)
    )
}
