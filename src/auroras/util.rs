//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use chrono::NaiveDateTime;


pub fn time_tag_to_i64(time_tag: &str) -> i64 {
    NaiveDateTime::parse_from_str(time_tag, "%Y-%m-%d %H:%M:%S%.3f").ok()
        .map(|dt| dt.and_utc().timestamp())
        .unwrap_or(0)
}
