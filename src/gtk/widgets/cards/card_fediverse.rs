//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use chrono::DateTime;

use crate::auroras::fediverse::tag::FediverseTag;

use super::super::cards::prelude::*;
use super::super::graphs::prelude::*;


const INTERVAL_FEDI_UPDATE: &str = "30"; // TODO: Verify

pub fn card_fediverse(tag: &FediverseTag) -> gtk4::Box {
    let value = tag.history
        .first()
        .map(|v| v.uses);

    // Add previous day to prevent big drop to 0
    // TODO: Use highest day?
    // calculate posts/hour for prev day and current and use last 24 hours?
    let value = tag.history
        .get(1)
        .map(|v| value.unwrap_or_default() + v.uses);

    let mut nodes: Vec<GraphNode> = tag.history
        .iter()
        .rev()
        .map(|day| GraphNode {
            value: day.accounts as f64,
            urgency: Urgency::Normal,
            info: Some(
                format!(
                    "{}  <b>{}</b>",
                    date_str(day.timestamp),
                    day.accounts,
                )
            ),
        })
        .collect();

    // TODO: Same length as Kp forecast
    let mut empty = vec![GraphNode::new(0.0); 17 - nodes.len()];
    empty.append(&mut nodes);

    let nodes = empty;

    let name = tag.name.clone();
    let url = tag.url.clone();
    let url = url.strip_prefix("https://").unwrap_or(&url);

    CardData {
        title: "Fediverse".into(),
        value: value.map(|v| v.to_string()),
        subtitle1: Some("people talking".into()),
        subtitle2: Some(format!("<a href='https://{url}' title='{url}'>#{name}</a>")),
        urgency: Urgency::Highlight,
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Bar,
                min: 0.0,
                max: 10.0, // TODO: max value? min 10.0
                gradient: None,
            }
        ),
        info: Some(
            InfoText {
                range: "∞".into(),
                tip: format!(
                    "Higher is better.\n\
                     Updated every {INTERVAL_FEDI_UPDATE} minutes."
                ),
                table: vec![
                    ("Average".into(), "~ 10".into()),
                    ("High".into(), "≥ 50".into()),
                ],
            }
        ),
    }.into()
}


fn date_str(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .map(|d| d.format("%a, %h %d").to_string())
        .unwrap_or_else(|| "—".to_string())
}
