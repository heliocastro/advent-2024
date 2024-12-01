// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod core;
mod day;

use crate::core::settings::Settings;
use crate::day::day1;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg[short, long, help = "Set data folder path", default_value = "data/"]]
    datadir: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run the day")]
    Day1,
}

fn main() {
    let cli = Cli::parse();
    let mut settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Error reading settings: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(data_dir) = cli.datadir.as_deref() {
        settings.datadir = data_dir.display().to_string();
    }

    match &cli.command {
        Some(Commands::Day1) => day1::day(&settings),
        None => {
            eprintln!("You not specifiedthe day");
            std::process::exit(1);
        }
    }
}
