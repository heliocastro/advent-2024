// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use std::{fs, io, path::PathBuf};

use crate::core::settings::Settings;

pub fn read_input(input: String, settings: &Settings) -> Result<String, io::Error> {
    let mut datadir: PathBuf = PathBuf::from(&settings.datadir);
    // Add the input file to the data directory
    datadir.push(input);
    if cfg!(debug_assertions) {
        println!(
            "Input requested: {}",
            datadir.as_path().display().to_string()
        );
    }

    fs::read_to_string(datadir)
}
