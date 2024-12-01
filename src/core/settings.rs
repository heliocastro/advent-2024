// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct Settings {
    #[serde(default = "default_datadir")]
    pub datadir: String,
}

fn default_datadir() -> String {
    "data/".to_string()
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(Environment::with_prefix("ADVENT"))
            .build()?;
        s.try_deserialize()
    }
}
