// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0
use crate::core::data::read_input;
use crate::core::pos::{Direction, Pos};
use crate::core::settings::Settings;
use crate::core::utils::print_result;
use log::{debug, info};

pub fn day(settings: &Settings) {
    let input: String = read_input("day6.txt", Some(&settings)).unwrap();

    info!("TODO ! {}", input.len());

    let (step1, step2) = process_data(&input);

    print_result(6, 1, step1.to_string());
    print_result(6, 2, step2.to_string());
}

pub fn process_data(input: &str) -> (usize, usize) {
    let mut labmap: Vec<Vec<char>> = Vec::new();
    let mut pos: Pos = Pos::new(0, 0);
    let mut sight: Direction = Direction::Right;
    let mut step1: usize = 0;
    let step2: usize = 0;

    for line in input.lines() {
        let current: Vec<char> = line.chars().collect();
        labmap.push(current);
        if let Some((index, guard_sight)) = guard_pos("<>^v", line) {
            pos = Pos::new(index as isize, (labmap.len() - 1) as isize);
            sight = guard_sight;
            debug!("Guard pos: {:?}, Guard sight: {:?}", pos, sight);
        }
    }

    let mut walked: Vec<Pos> = Vec::new();
    loop {
        if !move_on(&labmap, &mut pos, &mut sight, &mut walked, &mut step1) {
            break;
        }
    }

    for (rindex, row) in labmap.iter().enumerate() {
        for (cindex, column) in row.iter().enumerate() {
            let mut loop_matrix = labmap.clone();
            let loop_pos = Pos::new(rindex as isize, cindex as isize);
            match Pos::from_matrix(&loop_matrix, loop_pos) {
                Ok(value) => {
                    if value != '#' && pos != loop_pos {
                        loop_matrix[rindex][cindex] = '#';
                    }
                }
                Err(_) => continue,
            }
        }
        println!();
    }

    (step1, step2)
}

fn move_on(
    labmap: &Vec<Vec<char>>,
    pos: &mut Pos,
    sight: &mut Direction,
    walked: &mut Vec<Pos>,
    step: &mut usize,
) -> bool {
    match Pos::from_matrix(labmap, pos.move_dir(*sight)) {
        Ok(value) => {
            if value == '#' {
                match sight {
                    Direction::Right => *sight = Direction::Down,
                    Direction::Down => *sight = Direction::Left,
                    Direction::Left => *sight = Direction::Up,
                    Direction::Up => *sight = Direction::Right,
                    _ => {}
                }
                debug!("Direction: {:?}, Pos: {:?}", sight, pos);
            } else {
                // We already passed this point
                if !walked.contains(pos) {
                    walked.push(*pos);
                    *step += 1;
                    debug!("Walked: {:?}", walked);
                }
                *pos = pos.move_dir(*sight);
            }

            return true;
        }
        Err(e) => {
            debug!("Error: {}", e);
            info!("End of the line. too next step !");
            *step += 1;
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
    fn test_day6_step1() {
        init();
        let input: String = read_input("test_day6.txt", None).unwrap();
        let (result, _) = process_data(&input);
        assert_eq!(result, 41);
    }

    // #[test]
    // fn test_day5_step2() {
    //     init();
    //     let input: String = read_input("test_day5.txt", None).unwrap();
    //     let (_, result) = process_data(&input);
    //     assert_eq!(result, 123);
    // }
}
