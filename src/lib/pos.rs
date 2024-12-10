// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }

    pub fn move_dir(&self, dir: Direction) -> Pos {
        use Direction::*;
        match dir {
            Up => Pos::new(self.x, self.y - 1),
            Down => Pos::new(self.x, self.y + 1),
            Left => Pos::new(self.x - 1, self.y),
            Right => Pos::new(self.x + 1, self.y),
            UpLeft => Pos::new(self.x - 1, self.y - 1),
            UpRight => Pos::new(self.x + 1, self.y - 1),
            DownLeft => Pos::new(self.x - 1, self.y + 1),
            DownRight => Pos::new(self.x + 1, self.y + 1),
        }
    }

    pub fn surround(&self) -> &[Direction] {
        use Direction::*;
        &[Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight]
        // &[Right, Left, Down, Right]
    }

    pub fn from_matrix<T>(matrix: &Vec<Vec<T>>, pos: Pos) -> Result<T, String>
    where
        T: Clone + Copy,
    {
        if pos.y < 0
            || pos.y >= matrix.len() as isize
            || pos.x < 0
            || pos.x >= matrix[0].len() as isize
        {
            return Err(format!("Out of matrix boundary: {:?}.", pos));
        }
        Ok(matrix[pos.y as usize][pos.x as usize])
    }
}

pub trait FromUsize {
    fn from_usize(x: usize, y: usize) -> Self;
}

impl FromUsize for Pos {
    fn from_usize(x: usize, y: usize) -> Self {
        Pos::new(x as isize, y as isize)
    }
}
