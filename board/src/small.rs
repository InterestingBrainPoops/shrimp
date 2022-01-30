use crate::board::Coordinate;

#[derive(Clone, Debug)]
pub struct SmallRequest {
    pub turn: i64,
    pub board: SmallBoard,
    pub you: usize,
}

#[derive(Clone, Debug)]
pub struct SmallBoard {
    pub height: u8,
    pub width: u8,
    pub food: Vec<Coordinate>,
    pub hazards: Vec<Coordinate>,
    pub snakes: Vec<SmallBattlesnake>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmallBattlesnake {
    pub id: u8,
    pub health: u8,
    pub body: Vec<Coordinate>,
    pub head: Coordinate,
    pub length: u16,
    pub alive: bool,
}

impl SmallRequest {
    pub fn new() -> SmallRequest {
        SmallRequest {
            turn: 0,
            board: SmallBoard {
                height: 0,
                width: 0,
                food: vec![],
                hazards: vec![],
                snakes: vec![],
            },
            you: 0,
        }
    }
    pub fn amount_alive(&self) -> u8 {
        let mut out = 0;
        for snake in &self.board.snakes {
            if snake.alive {
                out += 1;
            }
        }
        out
    }
}
