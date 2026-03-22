//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gio;
use gtk4::{
    Align,
    Button,
    Label,
    Orientation,
};

use libadwaita::prelude::*;
use libadwaita::{
    ApplicationWindow,
    ButtonContent,
    Dialog,
    HeaderBar,
    Toast,
    ToastOverlay,
};


pub fn whatsnew_show_toast(overlay: &ToastOverlay, app_name: &str, version: &str) {
    // if { // TODO: compare to whatsnew-shown schema with version string. NOT on first release
    //  return;
    // }
    overlay.dismiss_all();
    overlay.add_toast(
        Toast::builder()
            .action_name("app.whatsnew")
            .button_label("Whatʼs New")
            .title(format!("Updated to {app_name} {version}"))
            .build()
    );

    // TODO: set whatsnew-shown to the version number here
}


pub fn whatsnew_show_dialog(parent: &ApplicationWindow, app_id: String, app_version: String, developer_name: String) {
    let dialog = Dialog::builder()
        .title("Whatʼs New")
        .width_request(360)
        .build();

    let header = HeaderBar::builder()
        .css_classes(["flat"])
        .build();

    let layout = gtk4::Box::new(Orientation::Vertical, 0);

    // TODO: Cards don't look good in light theme
    let card_update = card_new(
        &format!("This update — {app_version}"),
        vec![
            "First release!", // TODO: get from metainfo
            "Space weather forecast",
            "Written by a human using Rust"
        ],
        &button_releases(app_id)
    );

    let card_sponsor = card_new(
        "Future development",
        vec![
            "Notifications on high activity",
            "Location-based predictions",
            "Preferences"
        ],
        &button_sponsor(developer_name) // TODO: Add avatar here
    );

    let cards = gtk4::Box::new(Orientation::Vertical, 8);
    cards.set_margin_bottom(16);
    cards.append(&card_update);
    cards.append(&card_sponsor);

    layout.append(&header);
    layout.append(&cards);

    dialog.set_child(Some(&layout));
    dialog.present(Some(parent));
}


fn card_new(title: &str, changes: Vec<&str>, button: &Button) -> gtk4::Box
{
    let card = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_start(16)
        .margin_end(16)
        .css_classes(["card"])
        .build();

    let content = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_end(16)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .build();

    content.append(
        &Label::builder()
            .label(title)
            .css_classes(["heading"])
            .halign(Align::Start)
            .hexpand(true)
            .build()
    );

    content.append(&Label::new(None));

    for change in changes {
        content.append(
            &Label::builder()
                .label(format!("  •  {change}"))
                .halign(Align::Start)
                .hexpand(true)
                .build()
        );
    }

    content.append(&Label::new(None));
    content.append(button);

    card.append(&content);
    card
}


fn button_releases(app_id: String) -> Button {
    let button = Button::builder()
        .label("Releases")
        .build();

    button.connect_clicked(move |_| {
        let uri = format!("appstream:{}", app_id);
        _ = gio::AppInfo::launch_default_for_uri(&uri, None::<&gio::AppLaunchContext>);
    });

    button
}


fn button_sponsor(developer_name: String) -> Button {
    let content = ButtonContent::builder()
        .icon_name("emote-love-symbolic")
        .label(format!("Sponsor {developer_name}"))
        .build();

    let button = Button::builder()
        .child(&content)
        .css_classes(["suggested-action"])
        .build();

    button.connect_clicked(move |app| {
        _ = app.activate_action("app.sponsors", None);
    });

    button
}
