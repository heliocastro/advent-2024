// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::data::read_input;
use crate::core::settings::Settings;

pub fn day(settings: &Settings) {
    let input: String = read_input("day2.txt".to_string(), settings).unwrap();
    let safe_levels = parse_input(&input);
    println!("Day 2 - Step 2 result: {}", safe_levels);
}

fn parse_input(input: &String) -> i32 {
    let mut safe_levels: i32 = 0;

    for line in input.lines() {
        let mut previous: i32 = 0;
        let mut ascending: bool = true;
        line.split_whitespace().for_each(|x| {
            if previous == 0 {
                previous = x.parse::<i32>().unwrap();
            } else {
                let current = x.parse::<i32>().unwrap();
                let diff: i32 = if current > previous {
                    current - previous
                } else {
                    previous - current
                };
                if previous > x.parse::<i32>().unwrap()
                    && ascending
                    && current - previous >= 1
                    && (1..=3).contains(&diff)
                {
                    ascending = false;
                }
                previous = x.parse::<i32>().unwrap();
            }
        });
        println!("{:?}", 0);
    }

    safe_levels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_step1() {
        let settings = match Settings::new() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Error reading settings: {}", e);
                std::process::exit(1);
            }
        };
        let input: String = read_input("test_day2.txt".to_string(), &settings).unwrap();
        let safe_levels = parse_input(&input);
        assert_eq!(safe_levels, 2);
    }
}
