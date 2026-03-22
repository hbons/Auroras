//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gtk4::{
    gdk::Display,
    style_context_add_provider_for_display,
    Align,
    CssProvider,
    LevelBar,
    LevelBarMode,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use super::data::GraphData;


pub fn graph_level(data: &GraphData) -> LevelBar {
    let value = data.nodes
        .first()
        .cloned()
        .unwrap_or_default()
        .value;

    let level = LevelBar::builder()
        .mode(LevelBarMode::Discrete)
        .tooltip_text(
            format!(
                "{} / {}",
                value.round() as i32,
                data.max as i32
            )
        )
        .min_value(data.min)
        .max_value(data.max)
        .value(value.round())
        .valign(Align::End)
        .vexpand(true)
        .build();

    let min = data.min as i32;
    let max = data.max as i32;

    for i in min..max {
        level.add_offset_value(
            &format!("level-{}", i),
            i as f64
        );
    }

    level_add_css();
    level
}


fn level_add_css() {
    if let Some(display) = Display::default() {
        let provider = CssProvider::new();

        // TODO: Get from location/settings
        // Move logic elsewhere
        provider.load_from_string("
            .level-1, .level-2, .level-3, .level-4 {
                background-color: @accent_bg_color;
            }

            .level-5, .level-6, .level-7 {
                background-color: @warning_bg_color;
            }

            .level-8, .level-9 {
                background-color: @error_bg_color;
            }
            ",
        );

        style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
