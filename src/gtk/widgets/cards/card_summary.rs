//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use chrono::{
    DateTime,
    Local, // TODO: Use glib
};

use gio::Settings;

use gtk4::prelude::*;
use gtk4::{
    glib::timeout_add_seconds_local,
    glib::ControlFlow,
    Align,
    Button,
    Justification,
    Label,
    Orientation,
};

use libadwaita::{
    ButtonContent,
};

use crate::auroras::noaa::kpi::Kpi;
use crate::auroras::noaa::scales::geomagnetic::GeomagneticActivityScale;

use crate::gtk::widgets::cards::prelude::*;
use crate::gtk::widgets::cards::card::heading_new;
use crate::gtk::widgets::cards::card_kpi::kpi_info_table;


pub fn card_summary_new(kpi: &Kpi) -> gtk4::Box {
    let data = CardData {
        title: "Summary".into(),
        value: Some(GeomagneticActivityScale::from_kpi(kpi).to_string()),
        urgency: match kpi.value.round() as i32 { // TODO: From location/prefs
            0..=2 => Urgency::Normal,
            3 | 4 => Urgency::Highlight,
            5..=7 => Urgency::Alert,
            8 | 9 => Urgency::Critical,
            _ => Urgency::Normal,
        },
        info: Some(
            InfoText {
                range: "0–9".into(),
                tip: "Higher is better.\n\
                      Published every 3 hours.".into(),
                table: kpi_info_table(),
            }),
        ..Default::default()
    };

    let card = gtk4::Box::builder()
        .css_classes(["card", "activatable"])
        .build();

    let wrapper = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(16)
        .margin_top(6)
        .hexpand(true)
        .halign(Align::Fill)
        .build();


    let (time, date) = label_time_and_date();
    let (title, summary) = label_title_and_summary(&data, kpi);

    let source = label_source();

    let settings = Settings::new("studio.planetpeanut.Auroras");
    let timestamp = settings.int64("last-update");
    let updated = label_updated(timestamp);

    wrapper.append(&heading_new(&data));
    wrapper.append(&time);
    wrapper.append(&date);
    wrapper.append(&title);
    wrapper.append(&summary);
    wrapper.append(&updated);
    wrapper.append(&source);

    let time_clone = time.clone();
    let date_clone = date.clone();
    let update_clone = updated.clone();

    let settings = Settings::new("studio.planetpeanut.Auroras");

    timeout_add_seconds_local(1, move || {
        let now = Local::now();
        let last_update = settings.int64("last-update");

        time_clone.set_label(&time_str(&now));
        date_clone.set_label(&date_str(&now));
        update_clone.set_label(&updated_str(last_update));

        ControlFlow::Continue
    });

    card.append(&wrapper);
    card
}


fn label_time_and_date() -> (Label, Label) {
    let now = Local::now();

    let time = Label::builder()
        .label(time_str(&now))
        .css_classes(["numeric"])
        .tooltip_text("Current Time")
        .halign(Align::Center)
        .margin_top(40) // Align bottom of text with bottom of Kp LevelBar
        .use_markup(true)
        .build();

    let date = Label::builder()
        .label(date_str(&now))
        .tooltip_text("Current Date")
        .css_classes(["caption-heading", "dimmed"])
        .halign(Align::Center)
        .justify(Justification::Center)
        .margin_top(4)
        .use_markup(true)
        .build();

    (time, date)
}

fn time_str(dt: &DateTime<Local>) -> String {
    format!(
        "<span size='32pt' weight='700'>{}</span>",
        dt.format("%H:%M")
    )
}

fn date_str(dt: &DateTime<Local>) -> String {
    dt.format("%A, %B %-d").to_string()
}


fn label_title_and_summary(data: &CardData, kpi: &Kpi) -> (Button, Label) {
    let label = data.value.clone().unwrap_or("—".into());

    let button = Button::builder()
        .label(&label) // TODO: Show Kp popover on click
        .tooltip_markup("Current Rating")
        .margin_top(32)
        .hexpand(false)
        .halign(Align::Center)
        .build();

    button.set_css_classes(
        match data.urgency {
            Urgency::Normal    => &["pill"],
            Urgency::Highlight => &["pill", "accent"],
            Urgency::Alert     => &["pill", "numeric", "warning"],
            Urgency::Critical  => &["pill", "numeric", "error"],
            Urgency::HolySh_t  => &["pill", "numeric", "error", "holy-sh_t"],
        }
    );

    if data.urgency == Urgency::Alert ||
       data.urgency == Urgency::Critical {
        button.set_child(
            Some(
                &ButtonContent::builder()
                    .icon_name("dialog-warning")
                    .label(label)
                    .build()
            )
        );
    }

    let summary = Label::builder()
        .label(
            match kpi.value as i32 {
                0..=2 =>
                    "<i>Low chance of aurora\n\
                     outside arctic circle.</i>".into(),
                _ =>
                    format!(
                        "<i>Chance of aurora in\n{}</i>",
                        kpi_to_locations(kpi).join(" – ")
                    ),
            }
        )
        .tooltip_text("Based on geomagnetic latitude")
        .css_classes(["caption", "dimmed"])
        .margin_top(16)
        .halign(Align::Center)
        .justify(Justification::Center)
        .use_markup(true)
        .build();

    (button, summary)
}


fn label_updated(timestamp: i64) -> Label {
    Label::builder()
        .label(updated_str(timestamp))
        .css_classes(["caption", "dimmed", "numeric"])
        .halign(Align::Center)
        .valign(Align::End)
        .vexpand(true)
        .justify(Justification::Center)
        .use_markup(true)
        .build()
}

fn updated_str(timestamp: i64) -> String {
    // TODO: "Updated <b>just now</b>"
    DateTime::from_timestamp(timestamp, 0) // TODO: Use glib
        .map(|d| d.with_timezone(&Local))
        .map(|d| d.format("Updated at <b>%H:%M</b>").to_string())
        .unwrap_or_else(|| "—".to_string())
}


fn label_source() -> Label {
    let url = "swpc.noaa.gov";

    Label::builder()
        .label(format!("Source: <a title='{url}' href='https://www.{url}'>NOAA∕SWPC</a>"))
        .css_classes(["caption", "dimmed"])
        .halign(Align::Center)
        .valign(Align::End)
        // .vexpand(true)
        .margin_top(2)
        .margin_bottom(8)
        .justify(Justification::Center)
        .use_markup(true)
        .build()
}


pub fn kpi_to_locations(kpi: &Kpi) -> Vec<String> {
    match kpi.value as i32 {
        3 => vec!["Reykjavik".into(), "Trondheim".into(), "Oulu".into()],
        4 => vec!["Faroe Islands".into(), "Bergen".into(),], // TODO: Add latitude degrees
        5 => vec!["Oslo".into(), "Stockholm".into(), "Helsinki".into()],
        6 => vec!["Edinburgh".into(), "Gothenburg".into(), "Riga".into()],
        7 => vec!["London".into(), "Amsterdam".into(), "Berlin".into()],
        8 => vec!["Paris".into(), "Prague".into(), "Kyiv".into()],
        9 => vec!["Zurich".into(), "Vienna".into(), "Milan".into()],
        _ => vec![],
    }
}
