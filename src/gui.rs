//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;


pub trait Gui {
    // fn gui_init(&self) -> Result<(), Box<dyn Error>>;
    fn gui_run(&self) -> Result<(), Box<dyn Error>>;
    // fn gui_run_background(&self) -> Result<(), Box<dyn Error>>;
    // fn gui_set_autostart(&self, value: bool) -> Result<(), Box<dyn Error>>;

    // fn gui_show_main_window(&self) -> Result<(), Box<dyn Error>>;
    // fn gui_show_settings_window(&self) -> Result<(), Box<dyn Error>>;

    // fn gui_show_notification() -> Result<(), Box<dyn Error>>;
}
