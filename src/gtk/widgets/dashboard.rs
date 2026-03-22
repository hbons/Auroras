//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use gtk4::prelude::*;
use gtk4::{
    Align,
    Grid,
    PositionType,
};

use crate::auroras::noaa::kpi::Kpi;
use crate::auroras::noaa::imf::MagneticField;
use crate::auroras::noaa::plasma::Plasma;
use crate::auroras::fediverse::tag::FediverseTag;

use crate::gtk::widgets::cards::prelude::*;


const DASHBOARD_MARGIN: i32 = 12;

// TODO: Result. on error the window can use the error page or toaster error
pub fn dashboard_new(
    kpi: &Kpi,
    kpi_forecast: Vec<Kpi>,
    imfs: Vec<MagneticField>,
    plasmas: Vec<Plasma>,
    tag: &FediverseTag,
) -> Grid
{
    let card_summary = card_summary_new(kpi);
    let card_kpi_now = card_kpi_now(kpi);
    let card_kpi_forecast = card_kpi_forecast(kpi_forecast);
    let card_imf_bz = card_imf_bz(imfs.clone());
    let card_imf_bt = card_imf_bt(imfs.clone());
    let card_plasma_speed = card_plasma_speed(plasmas.clone());
    let card_fediverse = card_fediverse(tag);

    let grid = Grid::builder()
        .name("dashboard")
        .column_homogeneous(true)
        .column_spacing(8)
        .row_homogeneous(false)
        .row_spacing(8)
        .margin_top(1) // To fix card clipping
        .margin_bottom(DASHBOARD_MARGIN)
        .margin_start(DASHBOARD_MARGIN)
        .margin_end(DASHBOARD_MARGIN)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    // First column
    grid.attach(&card_summary, 0, 0, 1, 3);

    // Middle column
    grid.attach_next_to(&card_kpi_now, Some(&card_summary), PositionType::Right, 1, 1);
    grid.attach_next_to(&card_imf_bz, Some(&card_kpi_now), PositionType::Bottom, 1, 1);
    grid.attach_next_to(&card_imf_bt, Some(&card_imf_bz), PositionType::Bottom, 1, 1);

    // Last column
    grid.attach_next_to(&card_kpi_forecast, Some(&card_kpi_now), PositionType::Right, 1, 1);
    grid.attach_next_to(&card_plasma_speed, Some(&card_kpi_forecast), PositionType::Bottom, 1, 1);
    grid.attach_next_to(&card_fediverse, Some(&card_plasma_speed), PositionType::Bottom, 1, 1);

    grid
}
