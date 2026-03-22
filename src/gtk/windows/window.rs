//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;

use gtk4::prelude::*;
use gtk4::{
    gdk::Display,
    glib::ControlFlow,
    glib::spawn_future_local,
    style_context_add_provider_for_display,
    Align,
    Button,
    CssProvider,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use libadwaita::prelude::*;
use libadwaita::{
    Application,
    ApplicationWindow,
    HeaderBar,
    Spinner,
    StyleManager,
    ToastOverlay,
    ToolbarView,
};

use crate::gtk::widgets::dashboard::dashboard_new;
use crate::gtk::widgets::menu::main_menu_new;

use crate::auroras::data::Data;
use crate::auroras::auroras_fediverse::{auroras_fediverse, auroras_fediverse_get_data}; // TODO: auroras::prelude::* ?
use crate::auroras::auroras_imf::{auroras_imf, auroras_imf_get_data};
use crate::auroras::auroras_kpi::{auroras_kpi_current, auroras_kpi_get_data};
use crate::auroras::auroras_kpi::auroras_kpi_forecast;
use crate::auroras::auroras_plasma::{auroras_plasma, auroras_plasma_get_data};


const WINDOW_WIDTH: i32 = 892;
const WINDOW_HEIGHT: i32 = 628;

pub fn window_new(application: &Application) -> Result<ApplicationWindow, Box<dyn Error>>
{
    let window = ApplicationWindow::builder()
        .application(application)
        .title("Auroras")
        .default_width(WINDOW_WIDTH)
        .default_height(WINDOW_HEIGHT)
        .width_request(WINDOW_WIDTH)
        .height_request(WINDOW_HEIGHT)
        .resizable(true)
        .build();

    // TODO: error screen with retry button. symbolic app icon

    // window.add_css_class("devel"); // TODO
    window_add_css(&window);

    let header = HeaderBar::new();

    let (button, spinner) = refresh_button_new();
    header.pack_start(&button);
    header.pack_start(&spinner);
    header.pack_end(&main_menu_new());

    let overlay = ToastOverlay::new();
    overlay.set_child(Some(&big_spinner_new()));

    let toolbar_view = ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&overlay));

    // super::super::dialogs::whatsnew::whatsnew_show_toast(&overlay, "Auroras", "49.0.0");

    let button_clone = button.clone();
    let overlay_clone = overlay.clone();

    spawn_future_local(async move {
        let data = Data::from_directory(
            &crate::runtime::Runtime::default().app_cache_home
        ).unwrap();

        _ = auroras_fediverse_get_data(&data).await;
        _ = auroras_kpi_get_data(&data).await;
        _ = auroras_imf_get_data(&data).await;
        _ = auroras_plasma_get_data(&data).await;


        button_clone.set_sensitive(true);
        button_clone.action_set_enabled("app.refresh", true);

        let clamp = libadwaita::Clamp::builder()
            .name("dashboard-container")
            .maximum_size(WINDOW_WIDTH)
            .hexpand(true)
            .vexpand(true)
            .valign(Align::Start)
            .build();


        let kpi = auroras_kpi_current(&data).unwrap().unwrap(); // TODO: Crashed here
        let kpi_forecast = auroras_kpi_forecast(&data).unwrap();
        let imfs = auroras_imf(&data).unwrap();
        let plasmas = auroras_plasma(&data).unwrap();
        let tag = auroras_fediverse(&data).unwrap();

        clamp.set_child(
            Some(&dashboard_new(&kpi, kpi_forecast, imfs, plasmas, &tag))
        );

        overlay_clone.set_child(Some(&clamp));

        ControlFlow::Break
    });


    window.set_content(Some(&toolbar_view));

    Ok(window)
}


fn big_spinner_new() -> Spinner {
    Spinner::builder()
        .width_request(64)
        .build()
}


fn refresh_button_new() -> (Button, Spinner) {
    let button = Button::builder()
        .name("button-refresh")
        .action_name("app.refresh")
        .sensitive(false)
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Refresh")
        .build();

    button.action_set_enabled("app.about", false);

    let spinner = Spinner::builder()
        .name("spinner-refresh")
        .visible(false)
        .margin_top(1)
        .margin_start(9)
        .build();

    (button, spinner)
}


const WINDOW_CLASS_NAME: &str = "backdrop";

fn window_add_css(window: &ApplicationWindow) {
    let style_manager = StyleManager::default();

    if let Some(display) = Display::default() {
        let provider = CssProvider::new();

        provider.load_from_string(
            &format!("
                .{WINDOW_CLASS_NAME} {{
                    background-color: var(--sidebar-bg-color);
                }}
            "),
        );

        let window_clone = window.clone();

        style_manager.connect_dark_notify(move |style| {
            if style.is_dark() {
                window_clone.remove_css_class(WINDOW_CLASS_NAME);
            } else {
                window_clone.add_css_class(WINDOW_CLASS_NAME);
            }
        });

        style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    window.add_css_class(WINDOW_CLASS_NAME);
}
