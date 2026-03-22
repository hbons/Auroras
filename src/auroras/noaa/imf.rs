//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


#[derive(Clone, Debug)]
pub struct MagneticField {
    pub timestamp: i64,

    /// The X-component (nT) toward the Sun
    pub bx: Option<f64>,

    /// The Y-component (nT) in the dawn–dusk direction (positive toward dusk)
    pub by: Option<f64>,

    /// The Z-component (nT) north–south
    pub bz: Option<f64>,
}


impl MagneticField {
    /// The longitude angle
    pub fn longitude(&self) -> Option<f64> {
        if let (Some(bx), Some(by)) = (self.bx, self.by) {
            let mut longitude = by.atan2(bx) *
                (180.0 / std::f64::consts::PI);

            if longitude < 0.0 { // Wrap to 0..=360
                longitude += 360.0;
            }

            Some(longitude)
        } else {
            None
        }
    }

    /// The latitude angle
    pub fn latitude(&self) -> Option<f64> {
        if let (Some(bx), Some(by), Some(bz)) = (self.bx, self.by, self.bz) {
            let latitude = bz.atan2((bx.powi(2) + by.powi(2)).sqrt()) *
                (180.0 / std::f64::consts::PI);

            Some(latitude)
        } else {
            None
        }
    }
}


impl MagneticField {
    /// Total field magnitude (nT)
    pub fn bt(&self) -> Option<f64> {
        if let (Some(bx), Some(by), Some(bz)) = (self.bx, self.by, self.bz) {
            Some((bx.powi(2) + by.powi(2) + bz.powi(2)).sqrt())
        } else {
            None
        }
    }
}
