//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{
    Align,
    Orientation,
};

use gtk4::{
    gdk::Display,
    style_context_add_provider_for_display,
    CssProvider,
    Frame,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use crate::gtk::widgets::cards::data::Urgency;
use crate::gtk::widgets::graphs::data::GraphData;


const BAR_WIDTH: i32 = 10;
const BAR_MIN_HEIGHT: i32 = 2;
const BAR_UNIT_HEIGHT: f64 = 8.0;
const BAR_SPACING: i32 = 2;

// TODO: values as tuples (real value + secondary string)
pub fn graph_bar(data: &GraphData) -> gtk4::Box { // TODO: impl on CardData?
    let layout = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(BAR_SPACING)
        .height_request((data.max * BAR_UNIT_HEIGHT) as i32 + BAR_MIN_HEIGHT) // The minimum height // TODO: Magic number
        .homogeneous(true)
        .hexpand(true)
        .margin_top(BAR_UNIT_HEIGHT as i32)
        .valign(Align::End)
        .halign(Align::Fill)
        .build();

    for node in data.nodes.clone() {
        let bar = Frame::builder()
            .valign(Align::End)
            .halign(Align::Fill)
            .width_request(BAR_WIDTH)
            .height_request((node.value.abs() * BAR_UNIT_HEIGHT) as i32 + BAR_MIN_HEIGHT)
            .hexpand(true)
            .vexpand(false)
            .build();

        if let Some(tooltip) = node.info {
            bar.set_tooltip_markup(Some(&tooltip));
        }

        if node.value as i32 == 0 {
            bar.set_opacity(0.5);
            bar.set_sensitive(false);
        }

        bar.set_css_classes(
            match node.urgency { // TODO: Use node.gradient
                Urgency::Normal    => &["bar"],
                Urgency::Highlight => &["bar"],
                Urgency::Alert     => &["bar", "warning"],
                Urgency::Critical  => &["bar", "error"],
                Urgency::HolySh_t  => &["bar", "error", "holy-sh_t"],
            }
        );

        layout.append(&bar);
    }

    bar_add_css();

    layout
}


fn bar_add_css() {
    if let Some(display) = Display::default() {
        let provider = CssProvider::new();

        provider.load_from_string("
            .bar {
                background-color: @accent_bg_color;
                border: none;
                box-shadow: none;
                border-radius: 4px;
            }

            .bar:hover {
                border-color: rgba(255, 255, 255, 0.3);
                border-width: 1px;
                border-style: solid;
            }

            .bar.warning {
                background-color: @warning_color;
            }

            .bar.alert, .bar.holy-sh_t {
                background-color: @error_color;
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
