use crate::board::Coordinate;

/// A game request that has stripped out useless info.
/// A key point here is that the you member is the index of you in the snakes list itself.
#[derive(Clone, Debug)]
pub struct SmallRequest {
    /// current turn (not updated by make / unmake move)
    pub turn: i64,
    /// the board
    pub board: SmallBoard,
    /// the you index in the board.snakes array
    pub you: usize,
}

/// The board info with stripped out uselsess info
#[derive(Clone, Debug)]
pub struct SmallBoard {
    /// zobrist hash
    pub zobrist: u64,
    /// height of board in cells
    pub height: u8,
    /// width of board in cells
    pub width: u8,
    /// Vector of all food on the board
    pub food: Vec<Coordinate>,
    /// Vector of all hazard squares on the board
    pub hazards: Vec<Coordinate>,
    /// Vector of all snakes, dead and alive
    pub snakes: Vec<SmallBattlesnake>,
}

/// Small battlesnake is a snake that also cintains whether or not its alive.
/// This allows for a minor speedup where you only need to change a bool or check for a bool to determine whether or not a snake is alive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmallBattlesnake {
    /// The id, which lines up with the index in the snakes array
    pub id: u8,
    /// health of the snake
    pub health: u8,
    /// body vector
    pub body: Vec<Coordinate>,
    /// Position of head
    pub head: Coordinate,
    /// Length of snake
    pub length: u16,
    /// whether or not the snake is alive
    pub alive: bool,
}

impl Default for SmallRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl SmallRequest {
    /// make a empty smallreq
    pub fn new() -> SmallRequest {
        SmallRequest {
            turn: 0,
            board: SmallBoard {
                zobrist: 0,
                height: 0,
                width: 0,
                food: vec![],
                hazards: vec![],
                snakes: vec![],
            },
            you: 0,
        }
    }
    /// Get the amount of alive snakes
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
