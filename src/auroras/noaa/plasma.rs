//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


#[derive(Clone, Debug)]
pub struct Plasma {
    pub timestamp: i64,

    /// Number of particles per cm³
    pub density: Option<f64>,

    /// Speed of particles in km/s
    pub speed: Option<f64>,

    /// Temperature of particles in Kelvin
    pub temperature: Option<f64>,
}
