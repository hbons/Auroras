//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::sync::mpsc;
use std::thread;

use tokio::runtime::Runtime;

use gio::{
    Settings,
    SimpleAction,
};

use gtk4::prelude::*;
use gtk4::{
    glib::idle_add_local,
    glib::ControlFlow,
    glib::DateTime,
    Button,
    Widget,
};

use libadwaita::{
    Application,
    Clamp,
    Spinner,
};

use crate::auroras::data::Data;

use crate::auroras::auroras_fediverse::{auroras_fediverse, auroras_fediverse_get_data}; // TODO: auroras::prelude::* ?
use crate::auroras::auroras_imf::{auroras_imf, auroras_imf_get_data};
use crate::auroras::auroras_kpi::{auroras_kpi_current, auroras_kpi_get_data};
use crate::auroras::auroras_kpi::auroras_kpi_forecast;
use crate::auroras::auroras_plasma::{auroras_plasma, auroras_plasma_get_data};

use crate::gtk::widgets::dashboard::dashboard_new;


pub fn refresh_action(app: &Application) -> SimpleAction {
    app.set_accels_for_action("app.refresh", &["<Primary>r"]);

    let action = SimpleAction::new("refresh", None);
    let app_handle = app.clone();

    action.connect_activate(move |_, _| {
        if let Some(active_window) = app_handle.active_window() &&
           let Some(content) = active_window.child()
        {
            if let (Some(spinner), Some(button), Some(container)) =
                (widget_by_name("spinner-refresh", &content),
                 widget_by_name("button-refresh", &content),
                 widget_by_name("dashboard-container", &content))
            {
                let spinner = spinner.downcast::<Spinner>().unwrap();
                let button = button.downcast::<Button>().unwrap();
                let container = container.downcast::<Clamp>().unwrap();

                button.set_visible(false);
                spinner.set_visible(true);

                let settings = Settings::new("studio.planetpeanut.Auroras"); // TODO


                let (sender, receiver) = mpsc::channel::<()>();

                thread::spawn(move || {
                    let data = Data::from_directory(
                        &crate::runtime::Runtime::default().app_cache_home
                    ).unwrap();

                    if let Ok(runtime) = Runtime::new() {
                        _ = runtime.block_on(async {
                            // TODO: Run these in parallel:
                            _ = auroras_fediverse_get_data(&data).await;
                            _ = auroras_kpi_get_data(&data).await;
                            _ = auroras_imf_get_data(&data).await;
                            _ = auroras_plasma_get_data(&data).await;
                        });

                        _ = sender.send(());
                    }
                });

                idle_add_local(move || {
                    let data = Data::from_directory(
                        &crate::runtime::Runtime::default().app_cache_home
                    ).unwrap();

                    while let Ok(_) = receiver.try_recv() {
                        let kpi = auroras_kpi_current(&data).unwrap().unwrap(); // TODO: Crashed here
                        let kpi_forecast = auroras_kpi_forecast(&data).unwrap();
                        let imfs = auroras_imf(&data).unwrap();
                        let plasmas = auroras_plasma(&data).unwrap();
                        let tag = auroras_fediverse(&data).unwrap();

                        container.set_child(
                            Some(&dashboard_new(&kpi, kpi_forecast, imfs, plasmas, &tag))
                        );

                        button.set_visible(true);
                        spinner.set_visible(false);

                        if let Ok(now) = DateTime::now_local() {
                            let now = now.to_unix();
                            _ = settings.set_int64("last-update", now);
                        }
                    }

                    ControlFlow::Continue
                });
            }
        }
    });

    action
}


fn widget_by_name(name: &str, parent: &Widget) -> Option<Widget> {
    if parent.widget_name() == name {
        return Some(parent.clone());
    }

    let mut child = parent.first_child();

    while let Some(widget) = child {
        if let Some(found) = widget_by_name(name, &widget) {
            return Some(found);
        }

        child = widget.next_sibling();
    }

    None
}
