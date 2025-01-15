// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::settings::Settings;
use crate::utils::data::read_input;
use crate::utils::print::print_result;
use advent::pos::{Direction, Pos};
use advent::walked::Walked;
use log::{debug, info};

pub fn day(settings: &Settings) {
    let input: String = read_input("day6.txt", Some(&settings)).unwrap();

    let (step1, step2) = process_data(&input);

    print_result(6, 1, step1.to_string());
    print_result(6, 2, step2.to_string());
}

pub fn process_data(input: &str) -> (usize, usize) {
    let mut labmap: Vec<Vec<char>> = Vec::new();
    let mut pos: Pos = Pos::new(0, 0);
    let mut sight: Direction = Direction::Right;
    let mut step1: usize = 0;
    let mut step2: usize = 0;

    for line in input.lines() {
        let current: Vec<char> = line.chars().collect();
        labmap.push(current);
        if let Some((index, guard_sight)) = guard_pos("<>^v", line) {
            pos = Pos::new(index as isize, (labmap.len() - 1) as isize);
            sight = guard_sight;
            debug!("Guard pos: {:?}, Guard sight: {:?}", pos, sight);
        }
    }

    let original_pos = pos.clone();
    let original_sight = sight.clone();

    let mut walked: Vec<Walked> = Vec::new();
    loop {
        if !move_on(&labmap, &mut pos, &mut sight, &mut walked, &mut step1, None) {
            break;
        }
    }

    for (rindex, row) in labmap.iter().enumerate() {
        for (cindex, _) in row.iter().enumerate() {
            info!("{}{}", rindex, cindex);
            let mut loop_matrix = labmap.clone();
            pos = original_pos;
            sight = original_sight;
            walked.clear();

            loop_matrix[cindex][rindex] = '#';
            loop {
                if !move_on(
                    &loop_matrix,
                    &mut pos,
                    &mut sight,
                    &mut walked,
                    &mut step2,
                    Some(true),
                ) {
                    break;
                }
            }
        }
    }

    (step1, step2)
}

fn move_on(
    labmap: &Vec<Vec<char>>,
    pos: &mut Pos,
    sight: &mut Direction,
    walked: &mut Vec<Walked>,
    step: &mut usize,
    check_on_loop: Option<bool>,
) -> bool {
    let onloop = check_on_loop.unwrap_or(false);
    let mut walked_on = Walked {
        walked: *pos,
        direction: None,
    }; // We already passed this point
    match Pos::from_matrix(labmap, pos.move_dir(*sight)) {
        Ok(value) => {
            if value == '#' {
                walked_on.walked = pos.move_dir(*sight);
                walked_on.direction = Some(*sight);
                match sight {
                    Direction::Right => *sight = Direction::Down,
                    Direction::Down => *sight = Direction::Left,
                    Direction::Left => *sight = Direction::Up,
                    Direction::Up => *sight = Direction::Right,
                    _ => {}
                }
                if onloop {
                    if !walked.contains(&walked_on) {
                        walked.push(walked_on);
                        debug!("Added blocker {:?}", walked_on);
                    } else {
                        *step += 1;
                        return false;
                    }
                }

                debug!("Direction: {:?}, Pos: {:?}", sight, pos);
            } else {
                if !onloop {
                    if !walked.contains(&walked_on) {
                        walked.push(walked_on);
                        debug!("Added {:?}", walked_on);
                        *step += 1;
                    }
                }
                *pos = pos.move_dir(*sight);
            }

            return true;
        }
        Err(e) => {
            debug!("Error: {}", e);
            // if check on loop, this should always return true;
            if !onloop {
                *step += 1;
            }

            return false;
        }
    }
}

pub fn guard_pos(search_chars: &str, target_string: &str) -> Option<(usize, Direction)> {
    for (index, target_char) in target_string.chars().enumerate() {
        if search_chars.contains(target_char) {
            match target_char {
                '^' => return Some((index, Direction::Up)),
                'v' => return Some((index, Direction::Down)),
                '<' => return Some((index, Direction::Left)),
                '>' => return Some((index, Direction::Right)),
                _ => return None,
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn day6_step1() {
        init();
        let input: String = read_input("test_day6.txt", None).unwrap();
        let (result, _) = process_data(&input);
        assert_eq!(result, 41);
    }

    #[test]
    fn day6_step2() {
        init();
        let input: String = read_input("test_day6.txt", None).unwrap();
        let (_, result) = process_data(&input);
        assert_eq!(result, 6);
    }
}
