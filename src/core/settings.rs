// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct Settings {
    #[serde(default = "default_datadir")]
    pub datadir: String,
}

fn default_datadir() -> String {
    let mut datadir: PathBuf = env::current_dir().unwrap();
    datadir.push("data");
    if cfg!(debug_assertions) {
        println!("Datadir: {}", datadir.as_path().display().to_string());
    }
    datadir.as_path().display().to_string()
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(Environment::with_prefix("ADVENT"))
            .build()?;
        s.try_deserialize()
    }
}
