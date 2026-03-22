//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::fmt;
use super::scales::measurement::Measurement;


/// Planetary K-index
#[derive(Clone, Copy, Debug, Default)]
pub struct Kpi {
    pub timestamp: i64,

    /// KP-index (0–9)
    pub value: f64,

    /// Observed, estimated or predicted
    pub measurement: Option<Measurement>,
}


impl fmt::Display for Kpi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match (self.value.fract() * 100.0).round() as i32 {
                67 => format!("{}−", self.value.round()),
                33 => format!("{}+", self.value.round()),
                _ => self.value.round().to_string(),
            }
        )
    }
}
