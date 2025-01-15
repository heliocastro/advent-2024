// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use crate::core::settings::Settings;
use crate::utils::data::read_input;
use crate::utils::print::print_result;

use log::{debug, error};

pub fn day(settings: &Settings) {
    let input: String = read_input("day7.txt", Some(&settings)).unwrap();

    let (step1, step2) = process_data(&input);

    print_result(6, 1, step1.to_string());
    print_result(6, 2, step2.to_string());
}

#[derive(Debug, Clone, PartialEq)]
struct RopeBridge {
    test_value: i64,
    values: Vec<i32>,
}

pub fn process_data(input: &str) -> (i64, i64) {
    let mut step1: i64 = 0;
    let step2: i64 = 0;
    let mut bridge: Vec<RopeBridge> = Vec::new();

    for line in input.lines() {
        let value: Vec<&str> = line.split(":").collect();
        if value.len() != 2 {
            error!("Invalid input: {}", line);
            continue;
        }
        bridge.push(RopeBridge {
            test_value: value[0].parse::<i64>().unwrap(),
            values: value[1]
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        });
    }

    for entry in &bridge {
        let combinations = operation_combinations(entry.values.len() - 1);
        debug!("Combinations: {:?}", combinations);
        debug!("{:?}", entry.values);
        for combination in &combinations {
            let mut sum: i64 = entry.values[0] as i64;
            for (index, value) in entry.values.iter().enumerate().skip(1) {
                match combination.chars().nth(index - 1).unwrap() {
                    '+' => sum += *value as i64,
                    '*' => sum *= *value as i64,
                    _ => {
                        error!(
                            "Invalid operation: {}",
                            combination.chars().nth(index).unwrap()
                        );
                        continue;
                    }
                }
            }
            if sum as i64 == entry.test_value {
                step1 += sum as i64;
                debug!("Sum: {} = Test: {}", sum, entry.test_value);
                break;
            }
        }
    }

    (step1, step2)
}

fn operation_combinations(n: usize) -> Vec<String> {
    let base: Vec<char> = vec!['+', '*'];
    (0..base.len().pow(n as u32))
        .map(|i| {
            (0..n)
                .map(|j| base[(i / base.len().pow(j as u32)) % base.len()])
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day7_step1() {
        init();
        let input: String = read_input("test_day7.txt", None).unwrap();
        let (result, _) = process_data(&input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn day6_step2() {
        init();
        let input: String = read_input("test_day7.txt", None).unwrap();
        let (_, result) = process_data(&input);
        assert_eq!(result, 11387);
    }
}
