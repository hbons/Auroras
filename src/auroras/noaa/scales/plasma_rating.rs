//   Auroras, chase the lights
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::fmt;


#[derive(Debug, Clone, Copy)]
pub enum PlasmaSpeedRating {
    VeryHigh = 4,
    High = 3,
    Moderate = 2,
    Elevated = 1,
    Normal = 0,
}

impl PlasmaSpeedRating {
    pub fn from_speed(speed: u64) -> Self {
        match speed {
            s if s > 900 => Self::VeryHigh,
            s if s > 700 => Self::High,
            s if s > 500 => Self::Moderate,
            s if s > 400 => Self::Elevated,
            _  => Self::Normal,
        }
    }
}

impl fmt::Display for PlasmaSpeedRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_string();

        write!(f, "{}",
            match self {
                Self::VeryHigh => "Very High",
                _ => &s,
            }
        )
    }
}


#[derive(Debug, Clone, Copy)]
pub enum PlasmaDensityRating {
    VeryHigh = 3,
    High = 2,
    Moderate = 1,
    Low = 0,
}

impl PlasmaDensityRating {
    pub fn from_density(density: u64) -> Self {
        match density {
            d if d > 60 => Self::VeryHigh,
            d if d > 40 => Self::High,
            d if d > 20 => Self::Moderate,
            _  => Self::Low,
        }
    }
}

impl fmt::Display for PlasmaDensityRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_string();

        write!(f, "{}",
            match self {
                Self::VeryHigh => "Very High",
                _ => &s,
            }
        )
    }
}
