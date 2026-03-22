//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


#[derive(Clone, Copy, Debug, Default)]
pub enum GraphStyle {
    #[default]
    Bar,
    Line,
    Scale,
}
