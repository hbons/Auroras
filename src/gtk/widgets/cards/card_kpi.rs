//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use chrono::DateTime;

use crate::auroras::noaa::kpi::Kpi;
use crate::auroras::noaa::scales::geomagnetic::GeomagneticActivityScale;

use super::super::cards::prelude::*;
use super::super::graphs::prelude::*;


pub fn card_kpi_now(value: &Kpi) -> gtk4::Box {
    let kpi = *value;

    CardData {
        title: "K<sub>p</sub> – Now".into(),
        value: Some(kpi.to_string()),
        subtitle1: Some(format!("{} UTC", time_str(value.timestamp))),
        subtitle2: Some(GeomagneticActivityScale::from_kpi(value).to_long_string()),
        urgency: match kpi.value.round() as i32 {
            0..=2 => Urgency::Normal,
            3 | 4 => Urgency::Highlight,
            5..=7 => Urgency::Alert,
            8 | 9 => Urgency::Critical,
            _ => Urgency::Normal,
        },
        graph: Some(
            GraphData {
                nodes: vec![GraphNode::new(kpi.value)],
                style: GraphStyle::Scale,
                min: 0.0,
                max: 9.0,
                gradient: None,
            }
        ),
        info: Some(
            InfoText {
                range: "0–9".into(),
                tip: "Higher is better.\n\
                      Published every 3 hours.".into(), // TODO: Update expected in ~ 1h 13m
                table: kpi_info_table(),
            }
        ),
    }.into()
}


pub fn card_kpi_forecast(values: Vec<Kpi>) -> gtk4::Box {
    let nodes: Vec<GraphNode> = values
        .iter()
        .map(|v|
            GraphNode {
                value: v.value,
                urgency: match v.value as i32 { // TODO: From location/prefs
                    0..=2 => Urgency::Normal, // TODO: Shared fn with summary
                    3 | 4 => Urgency::Highlight,
                    5..=7 => Urgency::Alert,
                    8 | 9 => Urgency::Critical,
                    _ => Urgency::Normal,
                },
                info: Some(
                    format!(
                        "{}  <b>{}</b>", // TODO: local time
                        time_date_str(v.timestamp),
                        v,
                    ),
                )
            }
        )
        .collect();

    CardData {
        title: "K<sub>p</sub> – Coming Days".into(),
        value: None,
        subtitle1: None,
        subtitle2: None,
        urgency: Urgency::Normal,
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Bar,
                min: 0.0,
                max: 9.0,
                gradient: None,
            }
        ),
        info: Some(
            InfoText {
                range: "0–9".into(),
                tip: "Higher is better.".into(),
                table: kpi_info_table(),
            }
        ),
    }.into()
}


fn time_str(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|d| d.format("%H:%M").to_string())
        .unwrap_or_else(|| "—".to_string())
}

fn time_date_str(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|d| d.format("%a %H:%M").to_string())
        .unwrap_or_else(|| "—".to_string())
}


pub fn kpi_info_table() -> Vec<(String, String)> {
    let mut table: Vec<(String, String)> = vec![];

    for k in 0..=9 {
        let kpi = Kpi {
            value: k as f64,
            ..Default::default()
        };

        table.push((
            GeomagneticActivityScale::from_kpi(&kpi).to_long_string(),
            format!("= {k}")
        ));
    }

    table
}
