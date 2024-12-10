// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::utils::data::read_input;
use crate::core::settings::Settings;

struct DistancePair {
    origin: Vec<i32>,
    destination: Vec<i32>,
}

pub fn day(settings: &Settings) {
    let input: String = read_input("day1.txt", Some(&settings)).unwrap();
    let distance_pair = parse_input(&input);
    println!("Day 1 - Step 1 result: {}", calc_distance(&distance_pair));
    println!(
        "Day 1 - Step 2 result: {}",
        similarity_score(&distance_pair)
    );
}

fn parse_input(input: &String) -> DistancePair {
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

    distance_pair
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

fn similarity_score(distance_list: &DistancePair) -> i32 {
    let mut average: i32 = 0;
    for i in 0..distance_list.origin.len() {
        let filter: Vec<i32> = distance_list
            .destination
            .clone()
            .into_iter()
            .filter(|&x| x == distance_list.origin[i])
            .collect();
        average += distance_list.origin[i] * filter.len() as i32;
    }

    average
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_step1() {
        let input: String = read_input("test_day1.txt", None).unwrap();
        let distance_pair: DistancePair = parse_input(&input);
        let distance: i32 = calc_distance(&distance_pair);
        assert_eq!(distance, 11);
    }

    #[test]
    fn day1_step2() {
        let input: String = read_input("test_day1.txt", None).unwrap();
        let distance_pair: DistancePair = parse_input(&input);
        let average: i32 = similarity_score(&distance_pair);
        assert_eq!(average, 31);
    }
}
