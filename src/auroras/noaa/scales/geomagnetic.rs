//   Auroras, chase the lights
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::fmt;
use super::super::kpi::Kpi;


#[derive(Debug, Clone, Copy)]
pub enum GeomagneticActivityScale {
    /// Docs: https://www.swpc.noaa.gov/noaa-scales-explanation

    ExtremeStorm = 9,
    SevereStorm = 8,
    StrongStorm = 7,
    ModerateStorm = 6,
    MinorStorm = 5,
    Active = 4,
    Unsettled = 3,
    Moderate = 2,
    Low = 1,
    Quiet = 0,
}


impl GeomagneticActivityScale {
    pub fn from_kpi(kpi: &Kpi) -> Self {
        match kpi.value {
            k if k >= 9.0  => Self::ExtremeStorm,
            k if k >= 7.67 => Self::SevereStorm,
            k if k >= 6.67 => Self::StrongStorm,
            k if k >= 5.67 => Self::ModerateStorm,
            k if k >= 4.67 => Self::MinorStorm,
            k if k >= 3.67 => Self::Active,
            k if k >= 2.67 => Self::Unsettled,
            k if k >= 1.67 => Self::Moderate,
            k if k >= 0.67 => Self::Low,
            _ => Self::Quiet,
        }
    }
}


impl GeomagneticActivityScale {
    pub fn to_storm_scale(&self) -> Option<i32> {
        if *self as i32 >= Self::MinorStorm as i32 {
            Some(*self as i32 - 4)
        } else {
            None
        }
    }
}


impl fmt::Display for GeomagneticActivityScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_string();

        write!(f, "{}",
            match self {
                Self::ExtremeStorm  => "Extreme Storm",
                Self::SevereStorm   => "Severe Storm",
                Self::StrongStorm   => "Strong Storm",
                Self::ModerateStorm => "Moderate Storm",
                Self::MinorStorm    => "Minor Storm",
                _ => &s,
            }
        )
    }
}


impl GeomagneticActivityScale {
    pub fn to_long_string(&self) -> String {
        if let Some(scale) = self.to_storm_scale() {
            format!("G{scale} – {self}")
        } else {
            self.to_string()
        }
    }
}
