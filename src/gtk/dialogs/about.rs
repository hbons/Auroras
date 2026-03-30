//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gtk4::{
    License,
    Window,
};

use libadwaita::prelude::*;
use libadwaita::AboutDialog;


pub fn show_about_dialog(parent: &Window) {
    let about = AboutDialog::builder() // TODO: from_appdata() if running in Flatpak?
        .application_icon("studio.planetpeanut.Auroras")
        .application_name(env!["CARGO_PKG_NAME"])
        .developer_name("Hylke Bons")
        .version(env!["CARGO_PKG_VERSION"])
        .release_notes_version(env!["CARGO_PKG_VERSION"])
        .issue_url("https://github.com/hbons/Auroras/issues") // TODO
        .website(env!["CARGO_PKG_HOMEPAGE"])
        .copyright("Copyright © 2025–2026  Hylke Bons")
        .license_type(License::Agpl30)
        .build();

    // TODO
    // about.add_other_app("studio.planetpeanut.Bobby", "Bobby", "Browse SQLite files");

    about.present(Some(parent));
}
