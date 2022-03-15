use board::small::SmallRequest;
use evaluation::eval::StaticEval;
use movegen::{
    genmove::GenMove,
    makeunmake::{Direction, MakeUnmake, Move},
};

pub struct Evaluation {
    pub score: i32,
    pub direction: Option<Direction>,
}

pub trait Search {
    fn minimax(
        &mut self,
        depth: u8,
        alpha: i32,
        beta: i32,
        maximizing: bool,
        you_move: Option<Move>,
    ) -> Evaluation;
}

impl Search for SmallRequest {
    fn minimax(
        &mut self,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        you_move: Option<Move>,
    ) -> Evaluation {
        if self.amount_alive() as usize == 0 {
            // noone is alive
            return Evaluation {
                score: 0,
                direction: None,
            };
        } else if !self.board.snakes[self.you].alive {
            // im not alive
            return Evaluation {
                score: i32::MIN + 1000,
                direction: None,
            };
        } else if self.amount_alive() as usize == 1 {
            // im the only one left
            return Evaluation {
                score: i32::MAX - 1000,
                direction: None,
            };
        }

        if depth == 0 {
            return Evaluation {
                score: self.static_eval(),
                direction: None,
            };
        }

        if maximizing {
            let mut value = i32::MIN;
            let mut out = None;
            if self.snake_moves(self.you).is_empty() {
                // If you don't have any moves, return up with i32::MIN, since you are basically dead
                return Evaluation {
                    score: i32::MIN,
                    direction: Some(Direction::Up),
                };
            }
            for current_move in self.snake_moves(self.you) {
                let eval = self.minimax(depth, alpha, beta, !maximizing, Some(current_move));
                if value <= eval.score {
                    out = Some(current_move.direction);
                    value = eval.score;
                }
                if value >= beta {
                    break;
                }
                alpha = alpha.max(value);
            }
            Evaluation {
                score: value,
                direction: out,
            }
        } else {
            // let mut best_moves = vec![];
            let mut value = i32::MAX;

            for moves in &self.all_snake_moves(you_move.unwrap()) {
                let delta = self.make_move(moves);

                let eval = self.minimax(depth - 1, alpha, beta, !maximizing, None);
                self.unmake_move(&delta);

                if value >= eval.score {
                    // best_moves = moves.clone();
                    value = eval.score;
                }
                if value <= alpha {
                    break;
                }
                beta = beta.min(value);
            }
            Evaluation {
                score: value,
                direction: None,
            }
        }
    }
}
