use board::{
    board::Coordinate,
    boolboard::BoolBoard,
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

fn clear_bool_board_smart(bb: &mut BoolBoard, stuff: &Vec<Coordinate>) {
    for thing in stuff {
        bb[*thing] = false;
    }
}

impl Helpers for SmallRequest {
    fn move_snakes(&mut self, moves: &ArrayVec<[Move; SNAKE_MAX]>, delta: &mut Delta) {
        for snake_move in moves {
            // intermediate snake storage to prevent code duplication
            let mut snake = &mut self.board.snakes[snake_move.id as usize];

            // move the snakes head
            snake.head += Coordinate::from(snake_move.direction);
            // clear the old body from the boolboard
            clear_bool_board_smart(&mut snake.body_bb, &snake.body);

            // insert the new head into the beginning of the body
            snake.body.insert(0, snake.head);

            // generate the new snake body
            for coord in &snake.body[1..(snake.body.len() - 1)] {
                snake.body_bb[*coord] = true;
            }

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
            if self.board.food_bb[snake.head] {
                delta.prev_health.push((snake.id, snake.health));
                snake.body.push(*snake.body.last().unwrap());
                snake.health = 100;
                snake.length += 1;
                delta.eaten_food.push(snake.head);
            }
        }

        delta.eaten_food.dedup();
        for food in &delta.eaten_food {
            self.board
                .food
                .swap_remove(self.board.food.iter().position(|x| *x == *food).expect(""));
            self.board.food_bb[*food] = false;
        }
    }

    fn maybe_eliminiate_snakes(&mut self, delta: &mut Delta) {
        for snake in &mut self.board.snakes {
            if !snake.alive {
                continue;
            }
            if snake.health == 0 {
                snake.alive = false;
                delta.died.push(snake.id);
                continue;
            }

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

            if snake.body_bb[snake.head] {
                elims.push(snake.id);
                continue;
            }

            let mut has_body_collided = false;
            for other in &self.board.snakes {
                if !other.alive {
                    continue;
                }

                if other.id != snake.id && other.body_bb[snake.head] {
                    elims.push(snake.id);
                    has_body_collided = true;
                    break;
                }
            }
            if has_body_collided {
                continue;
            }

            let mut has_head_collided = false;

            for other in &self.board.snakes {
                if !other.alive {
                    continue;
                }
                if snake.id != other.id && other.head == snake.head && snake.length <= other.length
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
        elims.dedup();
        for id in elims {
            self.board.snakes[id as usize].alive = false;
            delta.died.push(id);
        }
        delta.died.dedup();
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
            self.board.food.push(*food);
            self.board.food_bb[*food] = true;
        }
        // bring back the dead
        for id in &delta.died {
            self.board.snakes[*id as usize].alive = true;
        }
        // unfeed snakes
        for (id, prev_health) in &delta.prev_health {
            self.board.snakes[*id as usize].health = *prev_health;
            self.board.snakes[*id as usize].body.pop().unwrap();
            self.board.snakes[*id as usize].length -= 1;
        }

        for snake in &mut self.board.snakes {
            if snake.alive {
                // increase health
                snake.health += 1;

                // clear the old body from the boolboard
                clear_bool_board_smart(&mut snake.body_bb, &snake.body);

                // unmove snakes
                snake.body.remove(0);
                snake.head = snake.body[0];
            }
        }

        for (id, tail) in &delta.tails {
            self.board.snakes[*id as usize].body.push(*tail);
        }
        for snake in &mut self.board.snakes {
            if snake.alive {
                // generate the new snake body
                for coord in &snake.body[1..] {
                    snake.body_bb[*coord] = true;
                }
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
