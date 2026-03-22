//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use std::fs;
use std::path::{ Path, PathBuf };
use std::time::Duration;

use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header::{ CONTENT_LENGTH, ETAG, IF_NONE_MATCH };
use reqwest::header::HeaderMap;
use serde_json::Deserializer;

use crate::log;


pub struct Data {
    pub data_dir: PathBuf,
}


impl Data {
    pub fn from_directory(path: &Path) -> Result<Self, Box<dyn Error>> {
        let data = Data {
            data_dir: path.to_path_buf(),
        };

        fs::create_dir_all(&data.data_dir)?;
        Ok(data)
    }


    pub const TIMEOUT: u64 = 5;

    pub async fn download(&self, url: &str, filename: &str) -> Result<(), Box<dyn Error>> {
        let etag_filename = Path::new(filename)
            .with_extension("etag")
            .to_string_lossy()
            .into_owned();

        let local_etag = self.read(&etag_filename).unwrap_or_default();

        let client = Client::builder()
            .connect_timeout(Duration::from_secs(Self::TIMEOUT))
            .build()?;

        let res = client
            .get(url)
            .header(IF_NONE_MATCH, local_etag)
            .timeout(Duration::from_secs(Self::TIMEOUT))
            .send()
            .await?;

        let headers = res.headers().clone();

        match res.status() {
            StatusCode::OK => {
                log::info(&format!("Data | Download | {url}"));

                let data = res.bytes().await?;
                self.check_size(&headers, &data)?;
                self.check_json(&data)?;
                self.store(Path::new(filename), &data)?;

                if let Some(etag) = headers.get(ETAG) {
                    self.store(Path::new(&etag_filename), etag.as_bytes())?;
                }

                Ok(())
            },
            StatusCode::NOT_MODIFIED => {
                log::info(&format!("Data | ✓ Cached | {url}"));
                Ok(())
            },
            status => {
                // dbg!(&headers);
                Err(status.to_string().into())
            },
        }
    }


    pub async fn download_with_fallbacks(&self, urls: &[&str], filename: &str) -> Result<(), Box<dyn Error>> {
        for url in urls {
            if self.download(url, filename).await.is_ok() {
                return Ok(());
            }

            log::info(&format!("Data | Download | Falling back to {url}"));
        }

        Err("No more URLs to try".into())
    }


    pub fn store(&self, relative_path: &Path, data: &[u8]) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(&self.data_dir)?;

        let path = self.data_dir.join(relative_path);
        log::info(&format!("Data | Store | {}", path.to_string_lossy()));

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, data)?;
        Ok(())
    }


    pub fn read(&self, filename: &str) -> Result<String, Box<dyn Error>> {
        let path = self.data_dir.join(filename);

        let data = fs::read_to_string(path)?;
        let data = data.trim_end_matches('\0'); // Data may have NULL padding

        Ok(data.to_string())
    }


    pub fn exists(&self) -> bool {
        fs::exists(&self.data_dir).unwrap_or(false)
    }


    fn check_size(&self, headers: &HeaderMap, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let size = headers.get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok()?.parse::<u64>().ok());

        match size {
            Some(s) if s as usize == data.len() => Ok(()),
            Some(_) | None => Err("Downloaded data incomplete".into()),
        }
    }


    fn check_json(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let iter = Deserializer::from_slice(data)
            .into_iter::<serde_json::Value>();

        for value in iter {
            value?;
        }

        Ok(())
    }
}
