//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use super::super::cards::data::Urgency;
use super::super::graphs::style::GraphStyle;


#[derive(Clone, Debug, Default)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub style: GraphStyle,
    pub min: f64,
    pub max: f64,

    /// Gradient with section urgencies (0.0 to 1.0)
    pub gradient: Option<Vec<(f64, f64, Urgency)>>,
}


#[derive(Clone, Debug, Default)]
pub struct GraphNode {
    pub value: f64,
    pub urgency: Urgency, // TODO: No longer needed
    pub info: Option<String>,
}

impl GraphNode {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}
