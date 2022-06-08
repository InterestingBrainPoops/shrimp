use board::{board::Coordinate, small::SmallRequest};

pub trait StaticEval {
    fn static_eval(&self) -> i32;
}

fn manhattan_dist(p1: &Coordinate, p2: &Coordinate) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

impl StaticEval for SmallRequest {
    fn static_eval(&self) -> i32 {
        let weights = (700, 5, 300, 30);
        let mut closest_pos = (&Coordinate::new(100, 100), 100);
        let mut biggest = true;
        for x in &self.board.snakes {
            if x.length >= self.board.snakes[self.you].length && x.id != self.you as u8 {
                biggest = false;
                break;
            }
        }
        if biggest {
            let mut smallest = (Coordinate::new(0, 0), 1000);
            for x in &self.board.snakes {
                if x.length < smallest.1 && x.id != self.you as u8 {
                    smallest.1 = x.length;
                    smallest.0 = x.head;
                }
            }
            closest_pos.1 = manhattan(&self.board.snakes[self.you].head, &smallest.0);
        } else {
            for food in &self.board.food {
                if closest_pos.1 > manhattan(food, &self.board.snakes[self.you].head) {
                    closest_pos.1 = manhattan(food, &self.board.snakes[self.you].head);
                    closest_pos.0 = food;
                }
            }
            if closest_pos.1 == 100 {
                closest_pos.1 = 0;
            }
        }
        let mut closest_snakehead = (&Coordinate::new(100, 100), 100);
        if !biggest {
            for snake in &self.board.snakes {
                if closest_snakehead.1 > manhattan(&snake.head, &self.board.snakes[self.you].head)
                    && snake.id != self.board.snakes[self.you].id
                {
                    closest_snakehead.1 = manhattan(&snake.head, &self.board.snakes[self.you].head);
                    closest_snakehead.0 = &snake.head;
                }
            }
            if closest_snakehead.1 == 100 {
                closest_snakehead.1 = 0;
            }
        }
        (self.board.snakes[self.you].length * weights.0 as u16) as i32
            - ((self.board.snakes.len() - amnt_dead(self)) * weights.1 as usize) as i32
            - closest_pos.1 * weights.2
            - closest_snakehead.1 * weights.3
    }
}

fn amnt_dead(board: &SmallRequest) -> usize {
    let mut out = 0;
    for snake in &board.board.snakes {
        if !snake.alive {
            out += 1;
        }
    }
    out
}
/// returns the manhattan distance between the 2 points.
fn manhattan(pos1: &Coordinate, pos2: &Coordinate) -> i32 {
    ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as i32
}
