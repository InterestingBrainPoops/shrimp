use std::ops::{Add, AddAssign};

use serde::Deserialize;

use crate::{
    boolboard::BoolBoard,
    small::{SmallBattlesnake, SmallRequest},
};

/// Input a game request
#[derive(Clone, Debug, Deserialize)]
pub struct GameRequest {
    pub turn: i64,
    board: BoardInfo,
    you: BattleSnake,
}
/// The board info itself
#[derive(Clone, Debug, Deserialize)]
struct BoardInfo {
    height: u8,
    width: u8,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<BattleSnake>,
}

/// A battlesnake
#[derive(Clone, Debug, Deserialize)]
struct BattleSnake {
    id: String,
    health: u8,
    body: Vec<Coordinate>,
    head: Coordinate,
    length: u16,
}

/// A coordinate
#[derive(Clone, Copy, Debug, Deserialize, PartialOrd, Ord, Default)]
pub struct Coordinate {
    /// x coordinate
    pub x: i32,
    /// y coordinate
    pub y: i32,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<Coordinate> for u128 {
    fn from(input: Coordinate) -> Self {
        let mut out = 1;
        out <<= input.y * 11;
        out <<= input.x;
        out
    }
}

impl From<u128> for Coordinate {
    fn from(input: u128) -> Self {
        let zeroes = input.trailing_zeros() as i32;
        Coordinate {
            x: zeroes % 11,
            y: zeroes / 11,
        }
    }
}

impl GameRequest {
    pub fn into_small(&self) -> SmallRequest {
        let mut out = SmallRequest::new();
        out.turn = self.turn;
        out.board.height = self.board.height;
        out.board.width = self.board.width;

        out.board.food = self.board.food.clone();
        out.board.hazards = self.board.hazards.clone();
        for (x, y) in self.board.snakes.iter().enumerate() {
            out.board.snakes.push(SmallBattlesnake {
                id: x as u8,
                health: y.health,
                body: y.body.clone(),
                head: y.head,
                alive: true,
                length: y.length,
                head_bb: BoolBoard::new(),
                body_bb: BoolBoard::new(),
            });
            // give the last added snake the body bits into its body bitboard
            // exclude the head because reasons
            for coord in &y.body[1..] {
                out.board.snakes.last_mut().unwrap().body_bb[*coord] = true
            }

            // give the last added snake the head bit into its head bitboard
            out.board.snakes.last_mut().unwrap().head_bb[y.head] = true;

            if y.id == self.you.id {
                out.you = x;
            }
        }

        for food in &self.board.food {
            out.board.food_bb[*food] = true;
        }
        out
    }
}
