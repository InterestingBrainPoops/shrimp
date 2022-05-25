// 2d array of booleans
// has a 1 block of buffer so that the greater than and less than conditions are usable

use std::ops::{Index, IndexMut};

use crate::board::Coordinate;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoolBoard {
    inner: [[bool; 13]; 13],
}

impl BoolBoard {
    pub fn new() -> Self {
        BoolBoard {
            inner: [[false; 13]; 13],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..13 {
            for x in 0..13 {
                self.inner[i][x] = false
            }
        }
    }
}
impl Default for BoolBoard {
    fn default() -> Self {
        Self::new()
    }
}
impl Index<Coordinate> for BoolBoard {
    type Output = bool;

    fn index(&self, index: Coordinate) -> &Self::Output {
        &self.inner[(index.y + 1) as usize][(index.x + 1) as usize]
    }
}
impl IndexMut<Coordinate> for BoolBoard {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        &mut self.inner[(index.y + 1) as usize][(index.x + 1) as usize]
    }
}
