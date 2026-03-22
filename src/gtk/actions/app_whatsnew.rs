//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gio::SimpleAction;

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
};

use crate::gtk::dialogs::whatsnew::whatsnew_show_dialog;


pub fn whatsnew_action(app: &Application) -> SimpleAction {
    let action = SimpleAction::new("whatsnew", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(active_window) = app_handle.active_window() &&
           let Ok(active_window) = active_window.downcast::<ApplicationWindow>() {
            whatsnew_show_dialog(
                &active_window,
                "studio.planetpeanut.Auroras".into(),
                "49.0.0".into(),
                "Hylke".into()
            );
        }
    });

    action
}
