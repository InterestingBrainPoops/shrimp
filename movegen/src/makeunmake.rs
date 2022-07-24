use board::{
    board::Coordinate,
    small::{SmallRequest, SNAKE_MAX},
};
use tinyvec::*;

/// A delta that stores the non-inferable data about a given state
pub struct Delta {
    /// Food that was eaten in the transition
    eaten_food: ArrayVec<[Coordinate; SNAKE_MAX]>,
    /// Snakes that died for a variety of reasons
    died: ArrayVec<[u8; SNAKE_MAX]>,
    /// Previous healths of snakes, (id, health)
    prev_health: ArrayVec<[(u8, u8); SNAKE_MAX]>,
    /// Tails of snakes that were there before, (id, position)
    tails: ArrayVec<[(u8, Coordinate); SNAKE_MAX]>,
}
/// Make and unmake move trait
pub trait MakeUnmake {
    /// Advance a state given a set of moves for each and all snakes
    fn make_move(&mut self, moves: &ArrayVec<[Move; SNAKE_MAX]>) -> Delta;
    /// Unmake a move given the delta of non-mirror moves
    fn unmake_move(&mut self, delta: &Delta);
}
trait Helpers {
    fn move_snakes(&mut self, moves: &ArrayVec<[Move; SNAKE_MAX]>, delta: &mut Delta);
    fn reduce_health(&mut self);
    fn maybe_feed_snakes(&mut self, delta: &mut Delta);
    fn maybe_eliminiate_snakes(&mut self, delta: &mut Delta);
}
/// Stores a move for a given id

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    /// Direction to move
    pub direction: Direction,
    /// ID of the snake
    pub id: u8,
}

impl Move {
    pub fn new(direction: Direction, id: u8) -> Self {
        Move { direction, id }
    }
}

/// Directions that the snakes can move
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

impl Helpers for SmallRequest {
    fn move_snakes(&mut self, moves: &ArrayVec<[Move; SNAKE_MAX]>, delta: &mut Delta) {
        for snake_move in moves {
            // intermediate snake storage to prevent code duplication
            let mut snake = &mut self.board.snakes[snake_move.id as usize];

            // move the snakes head
            snake.head += Coordinate::from(snake_move.direction);

            // insert the new head into the beginning of the body
            snake.body.insert(0, snake.head);

            // insert the current head into the body_bb
            snake.body_bb |= snake.head_bb;

            // add the new head and remove the old head from the head bitboard
            snake.head_bb ^= snake.head_bb | u128::from(snake.head);

            // remove the old tail
            snake.body_bb ^= u128::from(snake.body[snake.length as usize - 1]);

            // add in the new tail
            // TODO: Look into this and see if its actually supposed to be 1. cause this seems like a useless op
            snake.body_bb |= u128::from(snake.body[snake.length as usize - 1]);

            // update the turn delta
            delta.tails.push((
                snake_move.id,
                self.board.snakes[snake_move.id as usize]
                    .body
                    .pop()
                    .unwrap(),
            ));
        }
    }

    fn reduce_health(&mut self) {
        for snake in &mut self.board.snakes {
            if snake.alive {
                snake.health -= 1;
            }
        }
    }

    fn maybe_feed_snakes(&mut self, delta: &mut Delta) {
        for snake in &mut self.board.snakes {
            if (snake.head_bb & self.board.food_bb) != 0 {
                // preserve the current health of the snake
                delta.prev_health.push((snake.id, snake.health));
                // push the current tail again so that it has the same tail twice
                snake.body.push(*snake.body.last().unwrap());
                // set the new health to 100
                snake.health = 100;
                // increase the length
                snake.length += 1;
                // add the eaten food to the
                delta
                    .eaten_food
                    .push(Coordinate::from(snake.head_bb & self.board.food_bb));
            }
        }

        // sort so that dedup actually removes duplicates
        delta.eaten_food.sort_unstable();
        // remove consecutive duplicates
        delta.eaten_food.dedup();

        for food in &delta.eaten_food {
            // remove the eaten food from the board
            self.board
                .food
                .swap_remove(self.board.food.iter().position(|x| *x == *food).unwrap());
            // remove the eaten food from the food bb
            self.board.food_bb ^= u128::from(*food);
        }
    }

    fn maybe_eliminiate_snakes(&mut self, delta: &mut Delta) {
        for snake in &mut self.board.snakes {
            // just move on if the snake is not alive, none of this applies
            if !snake.alive {
                continue;
            }

            // snake has run out of health
            if snake.health == 0 {
                snake.alive = false;
                delta.died.push(snake.id);
                continue;
            }

            // snake has moved oob
            if snake.head.x >= self.board.width as i32
                || snake.head.x < 0
                || snake.head.y >= self.board.height as i32
                || snake.head.y < 0
            {
                snake.alive = false;
                delta.died.push(snake.id);
                continue;
            }
        }

        let mut elims: ArrayVec<[u8; SNAKE_MAX]> = array_vec![];

        for snake in &self.board.snakes {
            if !snake.alive {
                continue;
            }

            // check for self collisions
            if (snake.body_bb & snake.head_bb) != 0 {
                elims.push(snake.id);
                continue;
            }

            // check for body collision of other snakes
            let mut has_body_collided = false;
            for other in &self.board.snakes {
                // if the other guy isn't alive, then why bother
                if !other.alive {
                    continue;
                }

                // <make sure snake is not other> && < check whether snake has a body collision with other>
                if other.id != snake.id && ((other.body_bb & snake.head_bb) != 0) {
                    // add this snake to the elims
                    elims.push(snake.id);
                    // has body collided to true so that we dont check unnecessarily
                    has_body_collided = true;
                    break;
                }
            }
            if has_body_collided {
                continue;
            }

            let mut has_head_collided = false;

            for other in &self.board.snakes {
                // check if other is alive
                if !other.alive {
                    continue;
                }

                // check if snake isnt other && snake has the same head square as other && snake length <= other length
                if snake.id != other.id
                    && other.head_bb & snake.head_bb != 0
                    && snake.length <= other.length
                {
                    elims.push(snake.id);
                    has_head_collided = true;
                    break;
                }
            }

            if has_head_collided {
                continue;
            }
        }

        // sort so that dedup works
        elims.sort_unstable();
        // remove duplicates
        elims.dedup();
        for id in elims {
            // kill the snake
            self.board.snakes[id as usize].alive = false;
            // add the dead snake id to the died part of delta
            delta.died.push(id);
        }
    }
}
impl MakeUnmake for SmallRequest {
    fn make_move(&mut self, moves: &ArrayVec<[Move; SNAKE_MAX]>) -> Delta {
        let mut out = Delta {
            eaten_food: array_vec![],
            died: array_vec![],
            prev_health: array_vec![],
            tails: array_vec![],
        };
        self.move_snakes(moves, &mut out);

        self.reduce_health();

        self.maybe_feed_snakes(&mut out);

        self.maybe_eliminiate_snakes(&mut out);

        out
    }
    fn unmake_move(&mut self, delta: &Delta) {
        // put food back
        for food in &delta.eaten_food {
            // push the food into the food array
            self.board.food.push(*food);
            // add back the food into the food bb
            self.board.food_bb |= u128::from(*food);
        }
        // bring back the dead
        for id in &delta.died {
            // make the snake alive
            self.board.snakes[*id as usize].alive = true;
        }
        // unfeed snakes
        for (id, prev_health) in &delta.prev_health {
            // set the snakes health back to where it was before
            self.board.snakes[*id as usize].health = *prev_health;
            // remove the added body piece
            self.board.snakes[*id as usize].body.pop().unwrap();
            // reduce the length
            self.board.snakes[*id as usize].length -= 1;
        }
        // increase health
        for snake in &mut self.board.snakes {
            if snake.alive {
                snake.health += 1;
            }
        }
        // unmove snakes
        for snake in &mut self.board.snakes {
            if snake.alive {
                snake.body.remove(0);
                snake.head = snake.body[0];
                snake.head_bb = u128::from(snake.head);
            }
        }
        for (id, tail) in &delta.tails {
            self.board.snakes[*id as usize].body.push(*tail);
        }
        for snake in &mut self.board.snakes {
            if snake.alive {
                snake.body_bb ^= snake.head_bb;
                snake.body_bb |= u128::from(snake.body[snake.length as usize - 1]);
            }
        }
    }
}

impl From<Direction> for Coordinate {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Down => Coordinate { x: 0, y: -1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Right => Coordinate { x: 1, y: 0 },
        }
    }
}
impl Direction {
    pub fn to_string(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        }
    }
}
