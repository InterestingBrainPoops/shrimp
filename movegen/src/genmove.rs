use crate::{
    cart_prod::cartesian_product,
    makeunmake::{Direction, Move},
};
use board::{
    board::Coordinate,
    small::{SmallRequest, SNAKE_MAX},
};
use tinyvec::{array_vec, ArrayVec};
pub trait GenMove {
    /// Get valid moves for a given snake
    /// By valid that means it doesn't cause an instant death.
    fn snake_moves(&self, id: usize) -> ArrayVec<[Move; 4]>;
    /// Generate a 2D vector of moves for all snakes
    /// This should be given the move predetermined move for the "you" player
    fn all_snake_moves(
        &self,
        predet_move: Move,
    ) -> ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; 4_usize.pow(SNAKE_MAX as u32)]>;
}

fn get_neck_dir(head: &Coordinate, neck: &Coordinate) -> Option<Direction> {
    if head.x < neck.x {
        Some(Direction::Right)
    } else if head.x > neck.x {
        Some(Direction::Left)
    } else if head.y < neck.y {
        Some(Direction::Up)
    } else if head.y > neck.y {
        Some(Direction::Down)
    } else {
        None
    }
}

impl GenMove for SmallRequest {
    fn snake_moves(&self, id: usize) -> ArrayVec<[Move; 4]> {
        let mut out = array_vec![];

        if !self.board.snakes[id].alive {
            return out;
        }
        let possible_moves = array_vec![ [Move;4] =>
            Move::new(crate::makeunmake::Direction::Up,id as u8),
            Move::new(crate::makeunmake::Direction::Right,id as u8),
            Move::new(crate::makeunmake::Direction::Left, id as u8),
            Move::new(crate::makeunmake::Direction::Down,id as u8),
        ];
        for mov in possible_moves.iter() {
            let new_pos: Coordinate = self.board.snakes[id].head + mov.direction.into();
            let mut removed = false;
            let neck_direction =
                get_neck_dir(&self.board.snakes[id].head, &self.board.snakes[id].body[1]);
            match neck_direction {
                Some(neck_direction_actual) => {
                    if neck_direction_actual == mov.direction {
                        println!("{:?}", mov.direction);
                        removed = true;
                    } else {
                        for snake in &self.board.snakes {
                            if !snake.alive {
                                continue;
                            } // move on if the other snake is dead
                            if snake.id != mov.id
                                && snake.head == new_pos
                                && snake.length >= self.board.snakes[id].length
                            {
                                removed = true;
                                break;
                            } // remove the move if the head is the same as the new head pos, and the other length is bigger or equal to my length

                            if snake.body[1..((snake.length - 1) as usize)].contains(&new_pos) {
                                removed = true;
                                break;
                            } // remove if the head is in the other
                            if new_pos.x >= self.board.width as i32
                                || new_pos.x < 0
                                || new_pos.y >= self.board.height as i32
                                || new_pos.y < 0
                            {
                                removed = true;
                                break;
                            } // remove if the head is
                        }
                    }
                }
                None => {
                    removed = true;
                }
            }
            if !removed {
                out.push(*mov);
            }
        }
        out
    }

    fn all_snake_moves(
        &self,
        predet_move: Move,
    ) -> ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; 4_usize.pow(SNAKE_MAX as u32)]> {
        let mut moves: ArrayVec<[ArrayVec<[Move; SNAKE_MAX]>; SNAKE_MAX]> = array_vec![];
        for id in 0..self.board.snakes.len() {
            if id != self.you && self.board.snakes[id].alive {
                let generated_moves = self.snake_moves(id);
                if generated_moves.is_empty() {
                    moves.push(array_vec![[Move ; 4] => Move::new(Direction::Up, id as u8)])
                } else {
                    moves.push(generated_moves);
                }
            }
        }
        let x = array_vec![predet_move.clone()];
        moves.push(x);
        // println!("{:?}", moves);
        cartesian_product(moves)
    }
}
