use board::{board::Coordinate, small::SmallRequest};

pub trait StaticEval {
    fn static_eval(&self) -> i32;
}

fn manhattan_dist(p1: &Coordinate, p2: &Coordinate) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

impl StaticEval for SmallRequest {
    fn static_eval(&self) -> i32 {
        let mut closest_pos = (&Coordinate::new(100, 100), 100);
        let mut biggest = true;
        for x in &board.board.snakes {
            if x.length >= board.you.length && x.id != board.you.id {
                biggest = false;
                break;
            }
        }
        if biggest {
            let mut smallest = (Coordinate::new(0, 0), 1000);
            for x in &board.board.snakes {
                if x.length < smallest.1 && x.id != board.you.id {
                    smallest.1 = x.length;
                    smallest.0 = x.head;
                }
            }
            closest_pos.1 = manhattan(&board.you.head, &smallest.0);
        } else {
            for food in &board.board.food {
                if closest_pos.1 > manhattan(food, &board.you.head) {
                    closest_pos.1 = manhattan(food, &board.you.head);
                    closest_pos.0 = food;
                }
            }
            if closest_pos.1 == 100 {
                closest_pos.1 = 0;
            }
        }
        let mut closest_snakehead = (&Coordinate::new(100, 100), 100);
        if !biggest {
            for food in &board.board.snakes {
                if closest_snakehead.1 > manhattan(&food.head, &board.you.head)
                    && food.id != board.you.id
                {
                    closest_snakehead.1 = manhattan(&food.head, &board.you.head);
                    closest_snakehead.0 = &food.head;
                }
            }
            if closest_snakehead.1 == 100 {
                closest_snakehead.1 = 0;
            }
        }
        (board.you.length * weights.0 as u16) as i32
            - ((board.board.snakes.len() - amnt_dead(board)) * weights.1 as usize) as i32
            - closest_pos.1 * weights.2
            - closest_snakehead.1 * weights.3
    }
}
