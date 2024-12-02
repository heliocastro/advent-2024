// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use colored::Colorize;

pub fn print_result(day: i16, step: i16, result: String) {
    println!(
        "{} {} - {} {}: {}",
        "Day".white().bold(),
        day.to_string().white().bold(),
        "Step".white(),
        step.to_string().yellow().bold(),
        result.green().bold()
    );
}
