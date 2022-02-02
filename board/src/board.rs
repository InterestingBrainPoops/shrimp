use std::ops::{Add, AddAssign};

use serde::Deserialize;

use crate::small::{SmallBattlesnake, SmallRequest};

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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    /// x coordinate
    pub x: i32,
    /// y coordinate
    pub y: i32,
}

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
                head: y.head.clone(),
                alive: true,
                length: y.length,
            });

            if y.id == self.you.id {
                out.you = x;
            }
        }
        out
    }
}
