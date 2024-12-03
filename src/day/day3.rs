// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use regex::Regex;

use crate::core::data::read_input;
use crate::core::settings::Settings;
use crate::core::utils::print_result;

pub fn day(settings: &Settings) {
    let input: String = read_input("day3.txt".to_string(), settings).unwrap();
    let (step1, step2) = parse_input(&input);
    print_result(3, 1, step1.to_string());
    print_result(3, 2, step2.to_string());
}

fn parse_input(input: &String) -> (i32, i32) {
    let mut step1: i32 = 0;

    for line in input.lines() {
        step1 += simple_match(&line);
    }

    let step2: i32 = dont_do(input.replace("\n", "").as_str());

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

fn dont_do(input: &str) -> i32 {
    // Do frst don;t->do
    let mut re = Regex::new(r"don't\(()\).*?do\(\)").unwrap();
    let result = re.replace_all(input, "");
    // Do now dont to end of line
    re = Regex::new(r"don't\(\).*").unwrap();
    let result2 = re.replace_all(&result, "");
    return simple_match(&result2);
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

    #[test]
    fn test_day3_step2() {
        let settings = match Settings::new() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Error reading settings: {}", e);
                std::process::exit(1);
            }
        };
        let input: String = read_input("test_day3_2.txt".to_string(), &settings).unwrap();
        let (_, dont_do) = parse_input(&input);
        assert_eq!(dont_do, 48);
    }
}
