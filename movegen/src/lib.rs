/// Generate move functions
pub mod genmove;
/// Make and unmake move functions
pub mod makeunmake;
#[cfg(test)]
mod tests {
    use std::fs;

    use board::board::GameRequest;
    use board::small::SmallRequest;

    use crate::genmove::GenMove;
    use crate::makeunmake::{MakeUnmake, Move};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn perft_food_test() {
        // println!(
        //     "{}",
        //     env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json"
        // );
        // get file
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // run perft depth 4
        let t0 = small.clone();
        perft(&mut small, 5, true, None);
        assert!(small
            .board
            .food
            .iter()
            .all(|item| t0.board.food.contains(item)));
    }
    #[test]
    fn perft_wall_collision_test() {
        // println!(
        //     "{}",
        //     env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\wall.json"
        // );
        // get file
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // println!("{:?}", small);
        // run perft depth 4
        let t0 = small.clone();
        perft(&mut small, 5, true, None);
        // println!("{}, {}", small.board.snakes.len(), t0.board.snakes.len());
        assert!(small
            .board
            .food
            .iter()
            .all(|item| t0.board.food.contains(item)));
        assert_eq!(small.board.snakes, t0.board.snakes);
    }
    #[test]
    fn perft_body_collision_test() {
        // println!(
        //     "{}",
        //     env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\body.json"
        // );
        // get file
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // println!("{:?}", small);
        // run perft depth 4
        let t0 = small.clone();
        perft(&mut small, 5, true, None);
        // println!("{}, {}", small.board.snakes.len(), t0.board.snakes.len());
        assert!(small
            .board
            .food
            .iter()
            .all(|item| t0.board.food.contains(item)));
        assert_eq!(small.board.snakes, t0.board.snakes);
    }
    #[test]
    fn perft_head_collision_test() {
        // println!(
        //     "{}",
        //     env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\head.json"
        // );
        // get file
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // println!("{:?}", small);
        // run perft depth 4
        let t0 = small.clone();
        perft(&mut small, 5, true, None);
        // println!("{}, {}", small.board.snakes.len(), t0.board.snakes.len());
        assert!(small
            .board
            .food
            .iter()
            .all(|item| t0.board.food.contains(item)));
        assert_eq!(small.board.snakes, t0.board.snakes);
    }

    fn perft(state: &mut SmallRequest, depth: u8, player: bool, you_move: Option<Move>) {
        // println!("e");
        if depth == 0 || state.amount_alive() == 1 {
            return;
        }
        // println!("{:?}", state.snake_moves(state.you));
        if player {
            for x in state.snake_moves(state.you) {
                perft(state, depth, !player, Some(x));
            }
        } else {
            for x in state.all_snake_moves(you_move.unwrap()) {
                // let t0 = state.clone();
                let delta = state.make_move(x);
                perft(state, depth - 1, !player, None);
                state.unmake_move(&delta);
                // assert_eq!(state.board.snakes, t0.board.snakes);
            }
        }
    }
}
