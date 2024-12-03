// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use regex::Regex;

use crate::core::data::read_input;
use crate::core::settings::Settings;
use crate::core::utils::print_result;

pub fn day(settings: &Settings) {
    let input: String = read_input("day3.txt".to_string(), settings).unwrap();
    let (step1, _) = parse_input(&input);
    print_result(3, 1, step1.to_string());
    // print_result(2, 2, safe_levels_dampener.to_string());
}

fn parse_input(input: &String) -> (i32, i32) {
    let mut step1: i32 = 0;
    let mut step2: i32 = 0;

    for line in input.lines() {
        step1 += simple_match(&line);
    }

    (step1, step2)
}

fn simple_match(input: &str) -> i32 {
    let mut sum: i32 = 0;
    // Extract mul
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for result in re.captures_iter(input) {
        sum += &result[1].parse::<i32>().unwrap() * &result[2].parse::<i32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_step1() {
        let settings = match Settings::new() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Error reading settings: {}", e);
                std::process::exit(1);
            }
        };
        let input: String = read_input("test_day3.txt".to_string(), &settings).unwrap();
        let (simple_match, _) = parse_input(&input);
        assert_eq!(simple_match, 161);
    }

    // #[test]
    // fn test_day2_step2() {
    //     let settings = match Settings::new() {
    //         Ok(settings) => settings,
    //         Err(e) => {
    //             eprintln!("Error reading settings: {}", e);
    //             std::process::exit(1);
    //         }
    //     };
    //     let input: String = read_input("test_day2.txt".to_string(), &settings).unwrap();
    //     let (_, safe_levels_dampener) = parse_input(&input);
    //     assert_eq!(safe_levels_dampener, 4);
    // }
}
