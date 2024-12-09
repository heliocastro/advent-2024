// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::utils::data::read_input;
use crate::core::settings::Settings;
use crate::utils::print::print_result;
use log::{debug, info};

pub fn day(settings: &Settings) {
    let input: String = read_input("day5.txt", Some(&settings)).unwrap();

    info!("TODO ! {}", input.len());

    let (step1, step2) = process_data(&input);

    print_result(5, 1, step1.to_string());
    print_result(5, 2, step2.to_string());
}

pub fn process_data(input: &str) -> (i32, i32) {
    let mut page_rules: Vec<(i32, i32)> = Vec::new();
    let mut print_pages: Vec<Vec<i32>> = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            let mut parts = line.split("|");
            let part1 = parts.next().unwrap().parse::<i32>().unwrap();
            let part2 = parts.next().unwrap().parse::<i32>().unwrap();
            page_rules.push((part1, part2));
            continue;
        }

        if line.is_empty() {
            continue;
        }

        print_pages.push(line.split(',').map(|x| x.parse::<i32>().unwrap()).collect());
    }

    let mut step1: i32 = 0;
    let mut step2: i32 = 0;
    for mut pages in print_pages {
        let (mut status, mut page) = check_page(&pages, &page_rules, &mut step1);
        while !status {
            // Swap and try again as step 2
            let _prev = pages[page - 1];
            pages[page - 1] = pages[page];
            pages[page] = _prev;
            debug!("SWAPPED: {} {} {:?}", pages[page], pages[page - 1], pages);
            (status, page) = check_page(&pages, &page_rules, &mut step2);
        }
    }
    (step1, step2)
}

fn check_page(pages: &Vec<i32>, page_rules: &Vec<(i32, i32)>, step: &mut i32) -> (bool, usize) {
    debug!("-------- Pages: {:?}", pages);
    for page in 0..pages.len() {
        if page_rules.iter().any(|&(first, _)| first == pages[page]) {
            if page == 0 {
                continue;
            } else {
                for prev in 0..page {
                    let reverse_tuple = (pages[page], pages[prev]);
                    debug!("Check: {:?}", reverse_tuple);
                    if page_rules.contains(&reverse_tuple) {
                        debug!("{:?}", page_rules);
                        return (false, page);
                    }
                }
            }
        }
    }
    let middle = pages[(pages.len() - 1) / 2];
    debug!("Middle: {:?}", middle);
    *step += middle;
    (true, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day5_step1() {
        init();
        let input: String = read_input("test_day5.txt", None).unwrap();
        let (result, _) = process_data(&input);
        assert_eq!(result, 143);
    }

    #[test]
    fn day5_step2() {
        init();
        let input: String = read_input("test_day5.txt", None).unwrap();
        let (_, result) = process_data(&input);
        assert_eq!(result, 123);
    }
}
