// SPDX-FileCopyrightText: 2024 Helio Chissini de Castro
//
// SPDX-License-Identifier: GPL-2.0

use super::pos::{Direction, Pos};

#[derive(Default, Debug, Clone, Copy)]
pub struct Walked {
    pub walked: Pos,
    pub direction: Option<Direction>,
}

impl Walked {
    pub fn new() -> Self {
        Self {
            walked: Pos::new(0, 0),
            direction: None,
        }
    }
}

// Implement PartialEq for Walked
impl PartialEq for Walked {
    fn eq(&self, other: &Self) -> bool {
        self.walked == other.walked && self.direction == other.direction
    }
}
