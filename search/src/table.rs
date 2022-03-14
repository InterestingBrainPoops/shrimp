use std::ops::{Index, IndexMut};

use board::small::SmallBoard;
use movegen::makeunmake::Move;
const TT_SIZE: u64 = 1000000;

#[derive(Clone, PartialEq, Eq)]
enum Flag {
    Exact,
    Alpha,
    Beta,
    Nothing,
}

#[derive(Clone)]
struct Data {
    score: i32,
    best_move: Vec<Move>,
    flag: Flag,
    depth: u8,
}

impl Data {
    fn new() -> Self {
        Self {
            score: 0,
            best_move: vec![],
            flag: Flag::Nothing,
            depth: 0,
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

struct TranspositionTable {
    table: Vec<Data>,
    used: u64,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: vec![Default::default(); TT_SIZE as usize],
            used: 0,
        }
    }
}

impl Index<SmallBoard> for TranspositionTable {
    type Output = Data;

    fn index(&self, index: SmallBoard) -> &Self::Output {
        &self.table[(index.zobrist % TT_SIZE) as usize]
    }
}

impl IndexMut<SmallBoard> for TranspositionTable {
    fn index_mut(&mut self, index: SmallBoard) -> &mut Self::Output {
        &mut self.table[(index.zobrist % TT_SIZE) as usize]
    }
}
