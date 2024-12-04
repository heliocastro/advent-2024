// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use crate::core::data::read_input;
use crate::core::pos::{Direction, FromUsize, Pos};
use crate::core::settings::Settings;
use crate::core::utils::print_result;

pub fn day(settings: &Settings) {
    let input: String = read_input("day4.txt", Some(&settings)).unwrap();
    let matrix = create_matrix(input.as_str());

    let step1 = match_word(&matrix, "XMAS");
    let step2 = match_xmas(&matrix);
    print_result(4, 1, step1.to_string());
    print_result(4, 2, step2.to_string());
}

fn match_word(matrix: &Vec<Vec<char>>, word: &str) -> i32 {
    // Create vec with all lines
    let mut amount: i32 = 0;

    if let Some(first_letter) = word.chars().next() {
        let remaining = &word[1..];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] == first_letter {
                    let pos = Pos::from_usize(col, row);
                    // println!("\nFound initial letter {} at: {:?}", first_letter, pos);
                    // println!("{:?}", matrix[row]);
                    pos.surround().iter().for_each(|direction| {
                        walk_around(&matrix, &pos, remaining, &mut amount, *direction);
                    });
                }
            }
        }
    }

    amount
}

fn match_xmas(matrix: &Vec<Vec<char>>) -> i32 {
    // Create vec with all lines
    let first_letter = 'A';
    let mut amount = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == first_letter {
                let pos = Pos::from_usize(col, row);
                // println!("INITIAL POS: {:?}", pos);
                surround_me(matrix, &pos, &mut amount);
            }
        }
    }

    amount
}

fn surround_me(matrix: &Vec<Vec<char>>, pos: &Pos, amount: &mut i32) -> bool {
    let (width, height) = (matrix[0].len() as isize, matrix.len() as isize);
    let surround: Vec<Pos> = vec![
        pos.move_dir(Direction::UpLeft),
        pos.move_dir(Direction::UpRight),
        pos.move_dir(Direction::DownLeft),
        pos.move_dir(Direction::DownRight),
    ];

    for letter in "MS".chars() {
        let mut total = 0;
        for s in &surround {
            if s.x < 0 || s.y < 0 || s.x == width as isize || s.y == height as isize {
                // println!("SKIP LETTER: {}", letter);
                continue;
            }
            if matrix[s.y as usize][s.x as usize] == letter {
                // println!("--- {:?} {}", s, letter);
                total += 1;
            }
        }
        if total != 2 {
            return false;
        }
    }
    *amount += 1;
    return true;
}

fn walk_around(
    matrix: &Vec<Vec<char>>,
    pos: &Pos,
    word: &str,
    amount: &mut i32,
    direction: Direction,
) -> bool {
    let next_char = word.chars().nth(0);

    let npos = pos.move_dir(direction);

    if npos.x < 0
        || npos.y < 0
        || npos.y >= matrix.len() as isize
        || npos.x >= matrix[0].len() as isize
    {
        return false;
    }
    if let Some(next_char) = next_char {
        let remaining = &word[1..];
        if matrix[npos.y as usize][npos.x as usize] == next_char {
            walk_around(matrix, &npos, remaining, amount, direction);
        } else {
            return false;
        }
    }
    if word.len() == 1 {
        *amount += 1;
        return true;
    }
    false
}

fn create_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_step1() {
        let input: String = read_input("test_day4.txt", None).unwrap();
        let matrix = create_matrix(input.as_str());

        let result: i32 = match_word(&matrix, "XMAS");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_day4_step2() {
        let input: String = read_input("test_day4.txt", None).unwrap();
        let matrix = create_matrix(input.as_str());

        let result: i32 = match_xmas(&matrix);
        assert_eq!(result, 9);
    }
}
