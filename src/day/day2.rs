// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::settings::Settings;
use crate::utils::data::read_input;
use crate::utils::print::print_result;

enum Direction {
    Neutral,
    Ascending,
    Descending,
}

pub fn day(settings: &Settings) {
    let input: String = read_input("day2.txt", Some(&settings)).unwrap();
    let (safe_levels, safe_levels_dampener) = parse_input(&input);
    print_result(2, 1, safe_levels.to_string());
    print_result(2, 2, safe_levels_dampener.to_string());
}

fn parse_input(input: &String) -> (i32, i32) {
    let mut safe_levels: i32 = 0;
    let mut safe_levels_dampener: i32 = 0;

    for line in input.lines() {
        if safe_level(&line, false) {
            safe_levels += 1;
        }
        if safe_level(&line, true) {
            safe_levels_dampener += 1;
        }
    }

    (safe_levels, safe_levels_dampener)
}

fn validate_entry(entries: &Vec<i32>) -> bool {
    let mut direction = Direction::Neutral;
    let mut iter = entries.iter();

    let &previous = iter.next().expect("No empty entries allowed.");
    let mut previous = previous;

    while let Some(&current) = iter.next() {
        let diff: i32;
        if current == previous {
            return false;
        } else if current >= previous {
            if let Direction::Descending = direction {
                return false;
            }
            diff = current - previous;
            direction = Direction::Ascending;
        } else {
            if let Direction::Ascending = direction {
                return false;
            }
            diff = previous - current;
            direction = Direction::Descending;
        }

        if !(1..=3).contains(&diff) {
            return false;
        }

        previous = current;
    }
    true
}

fn safe_level(entry: &str, dampener: bool) -> bool {
    let mut parsed_entry: Vec<i32> = entry
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("parse error"))
        .collect();

    let mut result: bool = validate_entry(&mut parsed_entry);
    if dampener && !result {
        for i in 0..parsed_entry.len() {
            let mut leveled: Vec<i32> = parsed_entry.clone();
            leveled.remove(i);
            result = validate_entry(&leveled);
            if result {
                return result;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_step1() {
        let input: String = read_input("test_day2.txt", None).unwrap();
        let (safe_levels, _) = parse_input(&input);
        assert_eq!(safe_levels, 2);
    }

    #[test]
    fn day2_step2() {
        let input: String = read_input("test_day2.txt", None).unwrap();
        let (_, safe_levels_dampener) = parse_input(&input);
        assert_eq!(safe_levels_dampener, 4);
    }
}
