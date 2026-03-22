//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gtk4::{
    gdk::Display,
    gdk::RGBA,
    gdk::Texture,
    glib::Bytes,
    Align,
    ContentFit,
    CssProvider,
    Overflow,
    Picture,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    TextDirection,
    locale_direction,
    style_context_add_provider_for_display,
};

use libadwaita::{
    StyleManager,
};

use crate::gtk::widgets::cards::data::Urgency;

use super::data::{
    GraphData,
};


pub fn graph_line(data: &GraphData, width: usize, height: usize) -> Option<Picture> {
    let points = normalize(data);

    let svg = format_svg(
        points,
        width,
        height,
        data.gradient.clone(),
    );

    let texture = Texture::from_bytes(
        &Bytes::from(svg.as_bytes())
    ).ok()?;

    Some(
        Picture::builder()
            .paintable(&texture)
            .content_fit(ContentFit::Fill)
            .css_classes(["line"])
            .overflow(Overflow::Hidden)
            .halign(Align::Fill)
            // .tooltip_markup("Period: <b>2 days</b>") // TODO: Highest, lowest
            .margin_top(8)
            .build()
    )
}


// Normalize to x/y coordinates in 0.0 – 1.0 range
fn normalize(data: &GraphData) -> Vec<(f64, f64, Urgency)>
{
    let range_y =
        if data.min < 0.0 {
            data.max + data.min.abs()
        } else {
            data.max - data.min
        };

    let mut points = vec![];

    for (i, node) in data.nodes.iter().enumerate() {
        if node.value == 0.0 {
            dbg!("Skipped a point");
            continue;
        }

        let y = // Shift to positive
            if data.min < 0.0 {
                node.value + data.min.abs()
            } else {
                node.value
            };

        let x = i as f64 / data.nodes.len() as f64;
        let x = match locale_direction() {
            TextDirection::Rtl => 1.0 - x,
            _ => x,
        };

        let y = y / range_y;

        let x = x.clamp(0.0, 1.0);
        let y = y.clamp(0.02, 0.98);

        points.push((x, y, node.urgency));
    }

    points
}


fn format_svg(
    points: Vec<(f64, f64, Urgency)>,
    width: usize,
    height: usize,
    gradient: Option<Vec<(f64, f64, Urgency)>>,
) -> String
{
    let stroke_width = 20;
    // let circle_r = 24;

    // let last_x = data.nodes.len() * spacing;
    // let last_y = range_y - data.nodes.iter().last().unwrap().value;

    let mut lines: Vec<String> = vec![];

    for window in points.windows(2) {
        if let [(x1, y1, urgency1), (x2, y2, _urgency2)] = window {
            let y1 = 1.0 - y1; // SVG x-axis is mirrored
            let x1 = 1.0 - x1;

            let y2 = 1.0 - y2; // SVG x-axis is mirrored
            let x2 = 1.0 - x2;

            let x1 = x1 * width as f64;
            let x2 = x2 * width as f64;
            let y1 = y1 * height as f64;
            let y2 = y2 * height as f64;

            let stroke = if gradient.is_some() {
                "url(#gradient)"
            } else {
                &to_color(*urgency1)
            };

            lines.push(
                format!("\
                    <line \
                        x1='{x1}' \
                        y1='{y1}' \
                        x2='{x2}' \
                        y2='{y2}' \
                        stroke='{stroke}' \
                        stroke-width='{stroke_width}' \
                        stroke-linecap='round' \
                    />\
                    "
                )
            );
        }
    }

    let lines = lines.join("");

    let stops =
        if let Some(g) = gradient {
            format_svg_gradient(g)
        } else {
            "".into()
        };

    line_add_css();

    // TODO: back to polyline with fixed absolute background gradient.
    // let card define the stops. stops can also be classes

    format!(r#"
        <svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">
            <linearGradient
                id="gradient"
                gradientUnits="userSpaceOnUse"
                x1="0"
                y1="0"
                x2="0"
                y2="{height}"
            >
                {stops}
            </linearGradient>

            {lines}
        </svg>
        "#
        // <circle TODO
        //     cx="{last_x}"
        //     cy="{last_y}"
        //     r="{circle_r}"
        //     fill="white"
        // />
    )
}


const GRADIENT_FUZZ: i32 = 2;

fn format_svg_gradient(gradient: Vec<(f64, f64, Urgency)>) -> String {
    gradient
        .iter()
        .map(|v| {
                let offset1 = ((1.0 - v.0) * 100.0) as i32 - GRADIENT_FUZZ;
                let offset2 = ((1.0 - v.1) * 100.0) as i32 + GRADIENT_FUZZ;
                let color = to_color(v.2);

                format!(r#"
                    <stop offset="{offset2}%" stop-color="{color}" />
                    <stop offset="{offset1}%" stop-color="{color}" />
                    "#
                )
            }
        )
        .collect()
}


// TODO: colors.rs
fn to_color(urgency: Urgency) -> String {
    let accent = to_hex(
        &StyleManager::default()
            .accent_color()
            .to_rgba()
    );

    match urgency {
        Urgency::Normal    => accent,
        Urgency::Highlight => accent,
        Urgency::Alert     => "#cd9309".into(), // yellow
        Urgency::Critical  => "#c01c28".into(), // red
        Urgency::HolySh_t  => "purple".into(),
    }
}


fn to_hex(rgba: &RGBA) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        (rgba.red() * 255.0) as u8,
        (rgba.green() * 255.0) as u8,
        (rgba.blue() * 255.0) as u8,
    )
}


fn line_add_css() {
    if let Some(display) = Display::default() {
        let provider = CssProvider::new();

        provider.load_from_string("
            .line {
                border-radius: 6px;
                border-width: 1px;
                border-color: rgba(0, 0, 0, 0.2);
                background-color: rgba(0, 0, 0, 0.062);
                border-style: solid;
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
