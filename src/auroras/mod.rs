//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


pub mod fediverse {
    pub mod archive {
        pub mod archive_fediverse;
    }

    pub mod tag;
}

pub mod noaa {
    pub mod archive {
        pub mod compress;
        pub mod delete;
        pub mod insert;
        pub mod select;
        pub mod table;
    }

    pub mod scales {
        pub mod geomagnetic;
        pub mod measurement;
        pub mod imf_rating;
        pub mod plasma_rating;
    }

    pub mod oval;
    pub mod kpi;
    pub mod imf;
    pub mod plasma;
}

pub mod auroras_fediverse;
pub mod auroras_imf;
pub mod auroras_kpi;
pub mod auroras_oval;
pub mod auroras_plasma;

pub mod archive;
pub mod data;
pub mod util;
