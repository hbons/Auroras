//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use crate::gtk::widgets::cards::prelude::*;
use crate::gtk::widgets::graphs::prelude::*;


#[derive(Clone, Default)]
pub struct CardData {
    /// Used as the heading
    pub title: String,

    /// Featured number
    pub value: Option<String>,

    /// Mostly used for a unit
    pub subtitle1: Option<String>,

    /// Mostly used for a rating
    pub subtitle2: Option<String>,

    /// Affects styling
    pub urgency: Urgency,

    /// Numbers to use in the graph
    pub graph: Option<GraphData>,

    /// More details to disclose
    pub info: Option<InfoText>,
}

impl Into<gtk4::Box> for CardData {
    fn into(self) -> gtk4::Box {
        card_new(self)
    }
}


#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Urgency {
     #[default] Normal,
     Highlight,
     Alert,
     Critical,
     HolySh_t,
}


#[derive(Clone, Default)]
pub struct InfoText {
    pub range: String,
    pub tip: String,
    pub table: Vec<(String, String)>,
}
