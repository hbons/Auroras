//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


pub mod actions {
    pub mod prelude;
    pub mod app_about;
    pub mod app_refresh;
    pub mod app_whatsnew;
}

pub mod dialogs {
    pub mod about;
    pub mod preferences;
    pub mod whatsnew;
}

pub mod widgets {
    pub mod graphs {
        pub mod prelude;
        pub mod bar;
        pub mod line;
        pub mod level;
        pub mod colors;
        pub mod data;
        pub mod style;
    }

    pub mod cards {
        pub mod prelude;
        pub mod card;
        pub mod card_fediverse;
        pub mod card_imf;
        pub mod card_kpi;
        pub mod card_plasma;
        pub mod card_summary;
        pub mod data;
    }

    pub mod dashboard;
    pub mod menu;
}

pub mod windows {
    pub mod window;
}

pub mod lib;
