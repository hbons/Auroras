//   Auroras, chase the lights
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


#![allow(clippy::ptr_arg)]
#![allow(clippy::collapsible_if)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::wildcard_imports)]

pub mod runtime;
pub mod cli;
pub mod cli_mirror;
pub mod gtk;
pub mod gui;
pub mod log;

pub mod auroras;

// mod tests;


use std::env::args;
use std::error::Error;
use std::process::exit;

use crate::runtime::app_version;
use crate::runtime::{ Runtime, app_runs_as_root, app_runs_in_terminal };
use crate::gui::Gui;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::debug_base(&app_version());

    if app_runs_as_root() {
        log::error_and_exit("Cannot run as root")
    }

    // if app_runs_in_terminal() {
    //     let mut app = App::default();
    //     let args = args().collect();

    //     match app.cli_parse_args(&args).await {
    //         Ok(_)  => exit(0),
    //         Err(e) => log::error_and_exit(&e.to_string())
    //     };
    // }

    let app = Runtime::default();
    app.gui_run()?;

    Ok(())
}
