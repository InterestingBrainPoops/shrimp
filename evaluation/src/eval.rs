use board::{board::Coordinate, small::SmallRequest};

pub trait StaticEval {
    fn static_eval(&self) -> i32;
}

fn manhattan_dist(p1: &Coordinate, p2: &Coordinate) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

impl StaticEval for SmallRequest {
    fn static_eval(&self) -> i32 {
        let mut dist_food = i32::MAX;
        let you_head = self.board.snakes[self.you].head;
        let you_length = self.board.snakes[self.you].length;
        for food in &self.board.food {
            if manhattan_dist(&self.board.snakes[self.you].body[0], food) < dist_food {
                dist_food = manhattan_dist(&you_head, food);
            }
        }
        let mut biggest_length = true;
        for snake in &self.board.snakes {
            if snake.length >= you_length {
                biggest_length = false;
                break;
            }
        }
        (self.board.snakes[self.you].body.len()) as i32 * 10 - dist_food * 2
            + biggest_length as i32 * 5
    }
}
