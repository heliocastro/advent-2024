// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::data::read_input;
use crate::core::settings::Settings;

enum Direction {
    Neutral,
    Ascending,
    Descending,
}

pub fn day(settings: &Settings) {
    let input: String = read_input("day2.txt".to_string(), settings).unwrap();
    let safe_levels = parse_input(&input);
    println!("Day 2 - Step 2 result: {}", safe_levels);
}

fn parse_input(input: &String) -> i32 {
    let mut safe_levels: i32 = 0;

    for line in input.lines() {
        if safe_level(&line) {
            safe_levels += 1;
        }
    }

    safe_levels
}

fn safe_level(entry: &str) -> bool {
    let mut previous: Option<i32> = None;
    let mut direction = Direction::Neutral;

    for x in entry.split_whitespace() {
        let current = match x.parse::<i32>() {
            Ok(num) => num,
            Err(_) => return false,
        };

        if let Some(prev) = previous {
            let diff: i32;
            if current > prev {
                if let Direction::Descending = direction {
                    return false;
                }
                diff = current - prev;
                direction = Direction::Ascending;
            } else {
                if let Direction::Ascending = direction {
                    return false;
                }
                diff = prev - current;
                direction = Direction::Descending;
            }

            if !(1..=3).contains(&diff) {
                return false;
            }
        }
        previous = Some(current);
    }

    true
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
