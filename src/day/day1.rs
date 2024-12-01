// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::data::read_input;
use crate::core::settings::Settings;

struct DistancePair {
    origin: Vec<i32>,
    destination: Vec<i32>,
}

pub fn day(settings: &Settings) {
    let input: String = read_input("day1.txt".to_string(), settings).unwrap();
    println!("Day 1 - Step 1 result: {}", parse_input(&input))
}

fn parse_input(input: &String) -> i32 {
    let mut distance_pair: DistancePair = DistancePair {
        origin: Vec::new(),
        destination: Vec::new(),
    };

    for line in input.lines() {
        if let [origin_str, destination_str] = line.splitn(2, "   ").collect::<Vec<&str>>()[..] {
            if let (Ok(origin), Ok(destination)) =
                (origin_str.parse::<i32>(), destination_str.parse::<i32>())
            {
                distance_pair.origin.push(origin);
                distance_pair.origin.sort();
                distance_pair.destination.push(destination);
                distance_pair.destination.sort();
            }
        }
    }

    return calc_distance(&distance_pair);
}

fn calc_distance(distance_list: &DistancePair) -> i32 {
    let mut distance: i32 = 0;
    for i in 0..distance_list.origin.len() {
        if distance_list.destination[i] > distance_list.origin[i] {
            distance += distance_list.destination[i] - distance_list.origin[i];
        } else {
            distance += distance_list.origin[i] - distance_list.destination[i];
        }
    }

    return distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let settings = match Settings::new() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Error reading settings: {}", e);
                std::process::exit(1);
            }
        };
        let input: String = read_input("test_day1.txt".to_string(), &settings).unwrap();
        let distance: i32 = parse_input(&input);
        assert_eq!(distance, 11);
    }
}
