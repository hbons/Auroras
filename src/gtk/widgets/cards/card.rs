//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{
    Align,
    BaselinePosition,
    Button,
    Label,
    Orientation,
    Popover,
};

use crate::gtk::widgets::cards::prelude::*;
use crate::gtk::widgets::graphs::prelude::*;


// TODO: Impl on CardData? traits?
pub fn card_new(data: CardData) -> gtk4::Box {
    let card = gtk4::Box::builder()
        .css_classes(["card", "activatable"])
        .focusable(true)
        .accessible_role(gtk4::AccessibleRole::Heading)
        .build();

    let wrapper = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(16)
        .margin_top(6)
        .hexpand(true)
        .vexpand(true)
        .build();

    let content = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_start(16)
        .margin_end(16)
        .valign(Align::Fill)
        .vexpand(true)
        .build();

    let number1 = Label::builder()
        .label(
            format!(
                "<span size='26pt'><b>{}</b></span>",
                data.value.clone().unwrap_or("—".to_string())
            )
            .replace("-", "−")
        )
        .tooltip_markup(
            format!(
                "<b>{}</b> {}  {}",
                data.value.clone().unwrap_or("—".to_string()),
                data.subtitle1.clone().unwrap_or_default(),
                data.subtitle2.clone().unwrap_or_default()
            )
        )
        .halign(Align::Start)
        .valign(Align::BaselineCenter)
        .use_markup(true)
        .build();

    let subtitle = Label::builder()
        .label(data.subtitle1.clone().unwrap_or_default())
        .css_classes(["dimmed", "caption"])
        .halign(Align::Start)
        .tooltip_text("Unit")
        .use_markup(true)
        .build();

    let subtitle2 = Label::builder()
        .label(data.subtitle2.clone().unwrap_or_default())
        .css_classes(["caption-heading"])
        .halign(Align::Start)
        .tooltip_text("Rating")
        .use_markup(true)
        .build();

    subtitle2.add_css_class(
        match data.urgency {
            Urgency::Normal    => "dimmed",
            Urgency::Highlight => "accent",
            Urgency::Alert     => "warning",
            Urgency::Critical  => "error",
            Urgency::HolySh_t  => "holy-sh_t",
        }
    );

    if data.title == "Fediverse" {
        subtitle2.remove_css_class("caption-heading");
        subtitle2.add_css_class("caption");
    }

    let units = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .baseline_child(1)
        .baseline_position(BaselinePosition::Center)
        .valign(Align::BaselineCenter)
        .build();

    units.append(&subtitle);
    units.append(&subtitle2);

    let number = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .baseline_position(BaselinePosition::Center)
        .valign(Align::Start)
        .vexpand(true)
        .build();

    number.append(&number1);
    number.append(&units);

    if data.value.is_some() {
        content.append(&number);
    }

    if let Some(n) = data.value.clone() {
        if n == "1−" || n == "1" || n == "1+" {
            // Proportional "1" is too narrow alone
            number.add_css_class("numeric");
        }
    }

    if let Some(graph) = data.graph.clone() {
        match graph.style {
            GraphStyle::Bar => {
                let bar = graph_bar(&graph);
                content.append(&bar);
            },
            GraphStyle::Scale => {
                let scale = graph_level(&graph);
                content.append(&scale);
            },
            GraphStyle::Line => {
                let line = graph_line(&graph, 1600, 600);
                content.append(&line.unwrap())
            },
            // None => todo!(),
        };
    }

    wrapper.append(&heading_new(&data));
    wrapper.append(&content);

    card.append(&wrapper);
    card
}


pub fn heading_new(data: &CardData) -> gtk4::Box {
    let layout = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .focusable(true)
        .can_focus(true)
        .accessible_role(gtk4::AccessibleRole::Group)
        .name("TEST")
        .valign(Align::Start)
        .margin_start(16)
        .margin_end(6)
        .build();

    let label = Label::builder()
        .label(data.title.clone())
        .css_classes(["heading", "dimmed"])
        .halign(Align::Start)
        .hexpand(true)
        .use_markup(true)
        .build();

    layout.append(&label);
    layout.append(&info_button(data));
    layout
}


pub fn number_display_new() -> gtk4::Box {
    todo!();
}


pub fn info_button(data: &CardData) -> Button {
    let button = Button::builder()
        .icon_name("info-outline-symbolic")
        .tooltip_text("Details")
        .css_classes(["circular", "flat", "dimmed"])
        .halign(Align::Start)
        .build();

    let popover = info_popover(format_popover(&data.info.clone().unwrap()));
    popover.set_parent(&button);

    button.connect_clicked(move |_| {
        popover.popup();
    });

    button
}


fn info_popover(content: gtk4::Box) -> Popover {
    // let label = Label::builder()
    //     .label(&text)
    //     .wrap(true)
    //     .wrap_mode(gtk4::pango::WrapMode::WordChar)
    //     .max_width_chars(128)
    //     .width_request(100)
    //     .use_markup(true)
    //     .css_classes(vec!["numeric"])
    //     .build();

    // let wrap = gtk4::Box::new(Orientation::Vertical, 0);
    // wrap.set_width_request(200);
    // wrap.append(&label);

    Popover::builder()
        .child(&content)
        .width_request(100)
        .has_arrow(true)
        .position(gtk4::PositionType::Bottom)
        .build()
}


fn format_popover(text: &InfoText) -> gtk4::Box {
    let layout = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(12)
        .margin_end(12)
        .halign(Align::Start)
        .build();

    layout.append(
        &Label::builder()
            .label(format!("Range:  <b>{}</b>", text.range))
            .css_classes(["numeric"])
            .use_markup(true)
            .halign(Align::Start)
            .build()
    );

    layout.append(
        &Label::builder()
            .label(text.tip.clone())
            .css_classes(["caption", "dimmed"])
            .halign(Align::Start)
            .margin_bottom(
                if text.table.is_empty() {
                    0
                } else {
                    16
                }
            )
            .build()
    );

    for row in text.table.clone() {
        let line = gtk4::Box::new(Orientation::Horizontal, 0);

        let start = Label::builder()
            .label(format!("<b>{}</b>", row.0))
            .css_classes(["numeric"])
            .use_markup(true)
            .build();

        let end = Label::builder()
            .label(format!(" {}", row.1))
            .use_markup(true)
            .css_classes(["dimmed"])
            .build();

        line.append(&start);
        line.append(&end);

        layout.append(&line);
    }

    layout
}
