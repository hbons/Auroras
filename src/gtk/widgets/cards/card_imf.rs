//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use crate::auroras::noaa::imf::MagneticField;
use crate::auroras::noaa::scales::imf_rating::NorthSouthFieldRating;
use crate::auroras::noaa::scales::imf_rating::TotalFieldRating;

use super::super::cards::prelude::*;
use super::super::graphs::prelude::*;


const IMF_UPDATE_INTERVAL: &str = "10";


pub fn card_imf_bz(values: Vec<MagneticField>) -> gtk4::Box {
    let value = values
        .first()
        .map(|v| v.bz.unwrap_or_default());

    let rating = NorthSouthFieldRating::from_bz(
        value.unwrap_or_default()
    );

    let nodes: Vec<GraphNode> = values
        .iter()
        .map(|v| GraphNode {
                value: v.bz.unwrap_or_default(),
                urgency: to_urgency_bz(
                    NorthSouthFieldRating::from_bz(
                        v.bz.unwrap_or_default()
                    )
                ),
                info: None,
            }
        )
        .collect();

    CardData {
        title: "Interpl. Magn. Field – B<sub>z</sub>".into(),
        value: Some(value.map_or("—".into(), |n| format!("{:.1}", n))),
        subtitle1: Some("nT".into()),
        subtitle2: Some(rating.to_string()),
        urgency: to_urgency_bz(rating),
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Line,
                min: -25.0,
                max:  25.0, // TODO: use highest/lowest value +/-5? but with a minimum of -20
                gradient: Some(vec![
                    (0.5,   1.0,   Urgency::Normal),
                    (0.125, 0.5,   Urgency::Alert),
                    (0.0,   0.125, Urgency::Critical),
                ]),
            }
        ),
        info: Some(
            InfoText {
                range: "−25 to 25".into(),
                tip: format!(
                    "Negative is better.\n\
                     Updated every {IMF_UPDATE_INTERVAL} minutes."
                ),
                table: vec![
                    ("North".into(), "≥ 0".into()),
                    ("Weak South".into(), "&lt; 0".into()),
                    ("Moderate South".into(), "&lt; −5".into()),
                    ("Strong South".into(), "&lt; −10".into()),
                    ("Very Strong South".into(), "&lt; −20".into()),
                ],
            }
        ),
    }.into()
}


pub fn card_imf_bt(values: Vec<MagneticField>) -> gtk4::Box {
    let value = values.first().map(|v| v.bt().unwrap_or_default());

    let rating = TotalFieldRating::from_bt(
        value.unwrap_or_default()
    );

    let nodes: Vec<GraphNode> = values
        .iter()
        .map(|v| GraphNode {
                value: v.bt().unwrap_or_default(),
                urgency: to_urgency_bt(
                    TotalFieldRating::from_bt(
                        v.bt().unwrap_or_default()
                    )
                ),
                info: None,
            }
        )
        .collect();

    CardData {
        title: "Interpl. Magn. Field – B<sub>t</sub>".into(),
        value: Some(value.map_or("—".into(), |n| format!("{:.1}", n))),
        subtitle1: Some("nT".into()),
        subtitle2: Some(rating.to_string()),
        urgency: to_urgency_bt(rating),
        graph: Some(
            GraphData {
                nodes,
                style: GraphStyle::Line,
                min:  0.0,
                max: 30.0,
                gradient: Some(vec![
                    (0.66,  1.0, Urgency::Critical),
                    (0.33, 0.66, Urgency::Alert),
                    (0.0,  0.33, Urgency::Normal),
                ]),
            }
        ),
        info: Some(
            InfoText {
                range: "0−30+".into(),
                tip: format!(
                    "Higher is better.\n\
                     Updated every {IMF_UPDATE_INTERVAL} minutes."
                ),
                table: vec![
                    ("Very Strong".into(), "≥ 20".into()),
                    ("Strong".into(), "≥ 10".into()),
                    ("Moderate".into(), "≥ 5".into()),
                    ("Weak".into(), "≥ 0".into()),
                ],
            }
        ),
    }.into()
}


pub fn card_imf_bx(_values: Vec<MagneticField>) -> gtk4::Box {
    todo!()
}


pub fn card_imf_by(_values: Vec<MagneticField>) -> gtk4::Box {
    todo!()
}


fn to_urgency_bz(rating: NorthSouthFieldRating) -> Urgency {
    match rating {
        NorthSouthFieldRating::North           => Urgency::Normal,
        NorthSouthFieldRating::WeakSouth       => Urgency::Alert,
        NorthSouthFieldRating::ModerateSouth   => Urgency::Alert,
        NorthSouthFieldRating::StrongSouth     => Urgency::Alert,
        NorthSouthFieldRating::VeryStrongSouth => Urgency::Critical,
    }
}

fn to_urgency_bt(rating: TotalFieldRating) -> Urgency {
    match rating {
        TotalFieldRating::Weak       => Urgency::Normal,
        TotalFieldRating::Moderate   => Urgency::Highlight,
        TotalFieldRating::Strong     => Urgency::Alert,
        TotalFieldRating::VeryStrong => Urgency::Critical,
    }
}
