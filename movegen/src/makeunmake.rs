use board::{board::Coordinate, small::SmallRequest};
/// A delta that stores the non-inferable data about a given state
pub struct Delta {
    /// Food that was eaten in the transition
    eaten_food: Vec<Coordinate>,
    /// Snakes that died for a variety of reasons
    died: Vec<u8>,
    /// Previous healths of snakes, (id, health)
    prev_health: Vec<(u8, u8)>,
    /// Tails of snakes that were there before, (id, position)
    tails: Vec<(u8, Coordinate)>,
}
/// Make and unmake move trait
pub trait MakeUnmake {
    /// Advance a state given a set of moves for each and all snakes
    fn make_move(&mut self, moves: &Vec<Move>) -> Delta;
    /// Unmake a move given the delta of non-mirror moves
    fn unmake_move(&mut self, delta: &Delta);
}
trait Helpers {
    fn move_snakes(&mut self, moves: &Vec<Move>, delta: &mut Delta);
    fn reduce_health(&mut self);
    fn maybe_feed_snakes(&mut self, delta: &mut Delta);
    fn maybe_eliminiate_snakes(&mut self, delta: &mut Delta);
}
/// Stores a move for a given id

#[derive(Clone, Copy, Debug)]
pub struct Move {
    /// Direction to move
    pub direction: Direction,
    /// ID of the snake
    pub id: u8,
}

/// Directions that the snakes can move
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Helpers for SmallRequest {
    fn move_snakes(&mut self, moves: &Vec<Move>, delta: &mut Delta) {
        for snake_move in moves {
            self.board.snakes[snake_move.id as usize].head += snake_move.direction.into();
            let head = self.board.snakes[snake_move.id as usize].head.clone();
            self.board.snakes[snake_move.id as usize]
                .body
                .insert(0, head);
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
        for food in &self.board.food {
            let mut eaten = false;
            for snake in &mut self.board.snakes {
                if snake.head == food.clone() {
                    eaten = true;
                    delta.prev_health.push((snake.id, snake.health));
                    snake.body.push(snake.body.last().unwrap().clone());
                    snake.health = 100;
                    snake.length += 1;
                }
            }
            if eaten {
                delta.eaten_food.push(food.clone());
            }
        }
        for food in &delta.eaten_food {
            self.board.food.swap_remove(
                self.board
                    .food
                    .iter()
                    .position(|x| *x == food.clone())
                    .expect(""),
            );
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

        let mut elims = vec![];

        for snake in &self.board.snakes {
            if !snake.alive {
                continue;
            }

            if snake.body[1..].contains(&snake.head) {
                elims.push(snake.id);
                continue;
            }

            let mut has_body_collided = false;
            for other in &self.board.snakes {
                if !other.alive {
                    continue;
                }

                if other.id != snake.id && other.body[1..].contains(&snake.head) {
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
        for id in elims {
            self.board.snakes[id as usize].alive = false;
            delta.died.push(id);
        }
    }
}
impl MakeUnmake for SmallRequest {
    fn make_move(&mut self, moves: &Vec<Move>) -> Delta {
        let mut out = Delta {
            eaten_food: vec![],
            died: vec![],
            prev_health: vec![],
            tails: vec![],
        };
        self.move_snakes(moves, &mut out);

        self.reduce_health();

        self.maybe_feed_snakes(&mut out);

        self.maybe_eliminiate_snakes(&mut out);

        out
    }
    fn unmake_move(&mut self, delta: &Delta) {
        // bring back the dead
        for id in &delta.died {
            self.board.snakes[*id as usize].alive = true;
        }
        // unfeed snakes
        for (id, prev_health) in &delta.prev_health {
            self.board.snakes[*id as usize].health = *prev_health;
            self.board.snakes[*id as usize].body.pop();
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
                snake.head = snake.body[0].clone();
            }
        }
        for (id, tail) in &delta.tails {
            self.board.snakes[*id as usize].body.push(tail.clone());
        }
    }
}

impl Into<Coordinate> for Direction {
    fn into(self) -> Coordinate {
        match self {
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Down => Coordinate { x: 0, y: -1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Right => Coordinate { x: 1, y: 0 },
        }
    }
}