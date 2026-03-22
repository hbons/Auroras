//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use crate::auroras::noaa::plasma::Plasma;
use crate::auroras::noaa::scales::plasma_rating::{
    PlasmaDensityRating,
    PlasmaSpeedRating,
};

use super::super::cards::prelude::*;
use super::super::graphs::prelude::*;


pub fn card_plasma_speed(values: Vec<Plasma>) -> gtk4::Box {
    let value = values
        .first()
        .map(|i| i.speed.unwrap_or_default());

    let nodes: Vec<GraphNode> = values
        .iter()
        .map(|i| GraphNode::new(i.speed.unwrap_or_default()))
        .collect();

    let rating = PlasmaSpeedRating::from_speed(value.unwrap_or_default() as u64);

    CardData {
        title: "Solar Wind – Speed".into(),
        value: value.map(|v| format!("{:.0}", v)),
        subtitle1: Some("km/s".into()),
        subtitle2: Some(rating.to_string()),
        urgency: match rating {
            PlasmaSpeedRating::Normal   => Urgency::Normal,
            PlasmaSpeedRating::Elevated => Urgency::Highlight,
            PlasmaSpeedRating::Moderate => Urgency::Highlight,
            PlasmaSpeedRating::High     => Urgency::Alert,
            PlasmaSpeedRating::VeryHigh => Urgency::Critical,
        },
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Line,
                min: 0.0,
                max: 1000.0,
                gradient: Some(vec![
                    (0.9, 1.0, Urgency::Critical),
                    (0.7, 0.9, Urgency::Alert),
                    (0.0, 0.7, Urgency::Normal),
                ]),
            }
        ),
        info: Some(
            InfoText {
                range: "0–900+".into(),
                tip: tip_plasma(),
                table: vec![
                    ("Very High".into(), "≥ 900".into()),
                    ("High".into(), "≥ 700".into()),
                    ("Moderate".into(), "≥ 500".into()),
                    ("Elevated".into(), "≥ 400".into()),
                    ("Normal".into(), "≥ 0".into()),
                ],
            }
        ),
    }.into()
}


pub fn card_plasma_density(values: Vec<Plasma>) -> gtk4::Box {
    let value = values
        .first()
        .map(|i| i.density.unwrap_or_default());

    let nodes: Vec<GraphNode> = values
        .iter()
        .map(|i| GraphNode::new(i.density.unwrap_or_default()))
        .collect();

    let rating = PlasmaDensityRating::from_density(value.unwrap_or_default() as u64);

    CardData {
        title: "Solar Wind – Density".into(),
        value: value.map(|v| v.to_string()),
        subtitle1: Some("Kelvin".into()),
        subtitle2: Some(rating.to_string()),
        urgency: match rating {
            PlasmaDensityRating::Low      => Urgency::Normal,
            PlasmaDensityRating::Moderate => Urgency::Highlight,
            PlasmaDensityRating::High     => Urgency::Highlight,
            PlasmaDensityRating::VeryHigh => Urgency::Critical,
        },
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Line,
                min: 0.0,
                max: 100.0,
                gradient: None,
            }
        ),
        info: Some(
            InfoText {
                range: "0–60+".into(),
                tip: tip_plasma(),
                table: vec![
                    ("Very High".into(), "≥ 60".into()),
                    ("High".into(), "≥ 40".into()),
                    ("Moderate".into(), "≥ 20".into()),
                    ("Low".into(), "≥ 0".into()),
                ],
            }
        ),
    }.into()
}


pub fn card_plasma_temp(values: Vec<Plasma>) -> gtk4::Box {
    let value = values
        .first()
        .map(|i| i.temperature.unwrap_or_default());

    let nodes: Vec<GraphNode> = values
        .iter()
        .map(|i| GraphNode::new(i.temperature.unwrap_or_default() / 1000.0))
        .collect();

    CardData {
        title: "Solar Wind – Temperature".into(),
        value: value.map(|v| format!("{:.1}", v / 1000.0)),
        subtitle1: Some(" ∙ 10<sup>3</sup> K".into()),
        subtitle2: Some("Moderately High".into()), // TODO
        urgency: Urgency::Normal, // TODO
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Line,
                min: 0.0,
                max: 1000.0,
                gradient: None,
            }
        ),
        info: Some(
            InfoText {
                range: "0–1000+".into(),
                tip: tip_plasma(),
                table: vec![], // TODO
            }
        ),
    }.into()
}


const PLASMA_UPDATE_INTERVAL: &str = "10";

fn tip_plasma() -> String {
    format!("Higher is better.\n\
             Last 3 days.\n\
             Updated every {PLASMA_UPDATE_INTERVAL} minutes."
    )
}
