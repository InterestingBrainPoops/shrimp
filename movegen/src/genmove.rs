use crate::makeunmake::Move;
use board::{board::Coordinate, small::SmallRequest};
use permutator::copy::cartesian_product;
pub trait GenMove {
    fn snake_moves(&self, id: usize) -> Vec<Move>;
    fn all_snake_moves(&self, predet_move: Move) -> Vec<Vec<Move>>;
}

impl GenMove for SmallRequest {
    fn snake_moves(&self, id: usize) -> Vec<Move> {
        let mut out = vec![];

        if !self.board.snakes[id].alive {
            return out;
        }
        let possible_moves = vec![
            Move {
                direction: crate::makeunmake::Direction::Up,
                id: id as u8,
            },
            Move {
                direction: crate::makeunmake::Direction::Right,
                id: id as u8,
            },
            Move {
                direction: crate::makeunmake::Direction::Left,
                id: id as u8,
            },
            Move {
                direction: crate::makeunmake::Direction::Down,
                id: id as u8,
            },
        ];
        for mov in possible_moves.iter() {
            let new_pos: Coordinate = self.board.snakes[id].head.clone() + mov.direction.into();
            let mut removed = false;
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

                if snake.body[1..].contains(&new_pos) {
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
            if !removed {
                out.push(*mov);
            }
        }
        out
    }

    fn all_snake_moves(&self, predet_move: Move) -> Vec<Vec<Move>> {
        let mut moves: Vec<Vec<Move>> = vec![];
        for i in 0..self.board.snakes.len() {
            if i != self.you && self.board.snakes[i].alive {
                moves.push(self.snake_moves(i));
            }
        }
        let x = vec![predet_move];
        moves.push(x);
        // println!("{:?}", moves);
        let mut out = vec![];
        let slices: Vec<&[Move]> = moves.iter().map(Vec::as_slice).collect();
        cartesian_product(&slices[..], |x| out.push(x.to_vec()));
        out
    }
}
