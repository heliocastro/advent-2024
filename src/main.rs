// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use clap::{Parser, Subcommand};
use log::{error, info};
use std::path::PathBuf;

mod core;
mod day;
mod utils;

use crate::core::settings::Settings;
use crate::day::day1;
use crate::day::day2;
use crate::day::day3;
use crate::day::day4;
use crate::day::day5;
use crate::day::day6;
use crate::day::day7;

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
    #[command(about = "Run the day 1")]
    Day1,
    #[command(about = "Run the day 2")]
    Day2,
    #[command(about = "Run the day 3")]
    Day3,
    #[command(about = "Run the day 4")]
    Day4,
    #[command(about = "Run the day 5")]
    Day5,
    #[command(about = "Run the day 6")]
    Day6,
    #[command(about = "Run the day 7")]
    Day7,
}

fn main() {
    let cli = Cli::parse();

    // Init logger
    env_logger::init();

    let mut settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            error!("Error reading settings: {}", e);
            std::process::exit(1);
        }
    };

    if let Some(data_dir) = cli.datadir.as_deref() {
        settings.datadir = data_dir.display().to_string();
    }

    info!("Attempt to run Advent of Code 2024...");

    match &cli.command {
        Some(Commands::Day1) => day1::day(&settings),
        Some(Commands::Day2) => day2::day(&settings),
        Some(Commands::Day3) => day3::day(&settings),
        Some(Commands::Day4) => day4::day(&settings),
        Some(Commands::Day5) => day5::day(&settings),
        Some(Commands::Day6) => day6::day(&settings),
        Some(Commands::Day7) => day7::day(&settings),
        None => {
            error!("You not specified the day");
            std::process::exit(1);
        }
    }
}
