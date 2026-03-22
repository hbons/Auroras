//   Auroras, chase the lights
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::str;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Measurement {
    /// From measurement
    Observed,

    /// Awaiting processing
    Estimated,

    /// Forecasted future
    Predicted,
}


impl Measurement {
    pub fn to_short_string(&self) -> &str {
        match &self {
            Self::Observed  => "o",
            Self::Estimated => "e",
            Self::Predicted => "p",
        }
    }
}


impl str::FromStr for Measurement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "o" | "observed"  => Ok(Measurement::Observed),
            "e" | "estimated" => Ok(Measurement::Estimated),
            "p" | "predicted" => Ok(Measurement::Predicted),
            _ => Err("Invalid string".into()),
        }
    }
}
