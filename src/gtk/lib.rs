//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::time::Duration;
use std::error::Error;

use gio::Settings;

use gtk4::prelude::*;
use gtk4::{
    glib::timeout_add_seconds_local,
    glib::ControlFlow,
    glib::DateTime,
};

use libadwaita::Application;

use crate::runtime::Runtime;
use crate::gui::Gui;
use crate::log;

use crate::gtk::actions::prelude::*;
use crate::gtk::windows::window::window_new;


impl Gui for Runtime {
    fn gui_run(&self) -> Result<(), Box<dyn Error>> {
        let app = Application::builder()
            .application_id(&self.id)
            .build();

        app.connect_activate(|app| {
            if let Some(window) = app.active_window() {
                window.present();
            } else if let Ok(window) = window_new(app) {
                window.present();
            }
        });

        app.add_action(&about_action(&app));
        app.add_action(&refresh_action(&app));
        // app.add_action(&whatsnew_action(&app));

        start_auto_refresh(&app);
        app.run();

        Ok(())
    }
}


const INTERVAL_CHECK_SECONDS: u32 = 15;

fn start_auto_refresh(app: &Application) {
    let app_clone = app.clone();

    timeout_add_seconds_local(INTERVAL_CHECK_SECONDS, move || {
        let settings = Settings::new("studio.planetpeanut.Auroras"); // TODO
        let last_update = settings.int64("last-update");

        let interval = Duration::from_mins(
            settings.int64("interval") as u64
        );

        match DateTime::now_local() {
            Ok(now) => {
                let now = now.to_unix();

                if (now - last_update) >= interval.as_secs() as i64 {
                    app_clone.activate_action("refresh", None);
                }
            },
            Err(e) => log::error(
                &format!("Could not get local time: {e}")
            ),
        }

        ControlFlow::Continue
    });
}
