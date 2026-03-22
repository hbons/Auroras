//   Auroras, chase the lights
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NorthSouthFieldRating {
    VeryStrongSouth = 4,
    StrongSouth = 3,
    ModerateSouth = 2,
    WeakSouth = 1,
    North = 0,
}

impl NorthSouthFieldRating {
    pub fn from_bz(bz: f64) -> Self {
        match bz {
            b if b < -20.0 => Self::VeryStrongSouth,
            b if b < -10.0 => Self::StrongSouth,
            b if b <  -5.0 => Self::ModerateSouth,
            b if b <   0.0 => Self::WeakSouth,
            _  => Self::North,
        }
    }
}

impl fmt::Display for NorthSouthFieldRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self {
                Self::VeryStrongSouth => "Very Strong South",
                Self::StrongSouth     => "Strong South",
                Self::ModerateSouth   => "Moderate South",
                Self::WeakSouth       => "Weak South",
                Self::North           => "North",
            }
        )
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TotalFieldRating {
    VeryStrong = 3,
    Strong = 2,
    Moderate = 1,
    Weak = 0,
}

impl TotalFieldRating {
    pub fn from_bt(bt: f64) -> Self {
        match bt {
            b if b >= 20.0 => Self::VeryStrong,
            b if b >= 10.0 => Self::Strong,
            b if b >=  5.0 => Self::Moderate,
            _  => Self::Weak,
        }
    }
}

impl fmt::Display for TotalFieldRating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_string();

        write!(f, "{}",
            match self {
                Self::VeryStrong => "Very Strong",
                _ => &s,
            }
        )
    }
}
