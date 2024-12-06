// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::data::read_input;
use crate::core::settings::Settings;
use crate::core::utils::print_result;

pub fn day(settings: &Settings) {
    let input: String = read_input("day5.txt", Some(&settings)).unwrap();

    println!("TODO ! {}", input.len());

    let step1 = 0;
    let step2 = 0;
    print_result(5, 1, step1.to_string());
    print_result(5, 2, step2.to_string());
}
