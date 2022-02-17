/// Generate move functions
pub mod genmove;
/// Make and unmake move functions
pub mod makeunmake;

mod cart_prod;
#[cfg(test)]
mod tests {
    use std::fs;

    use board::board::GameRequest;
    use board::small::SmallRequest;
    use tinyvec::array_vec;

    use crate::genmove::GenMove;
    use crate::makeunmake::{MakeUnmake, Move};
    use pretty_assertions::assert_eq;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn perft_food_test() {
        // get file
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // run perft depth 4
        let mut t0 = small.clone();
        perft(&mut small, 5, true, None);
        t0.board.food.sort();
        small.board.food.sort();
        assert_eq!(t0.board.food, small.board.food);
    }
    #[test]
    fn perft_wall_collision_test() {
        // println!(
        //     "{}",
        //     env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\wall.json"
        // );
        // get file
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\wall.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // println!("{:?}", small);
        // run perft depth 4
        let mut t0 = small.clone();
        perft(&mut small, 5, true, None);
        t0.board.food.sort();
        small.board.food.sort();
        assert_eq!(t0.board.food, small.board.food);
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
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\body.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // println!("{:?}", small);
        // run perft depth 4
        let mut t0 = small.clone();
        perft(&mut small, 5, true, None);
        t0.board.food.sort();
        small.board.food.sort();
        assert_eq!(t0.board.food, small.board.food);
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
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\head.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let mut small = seralized.into_small();
        // println!("{:?}", small);
        // run perft depth 4
        let mut t0 = small.clone();
        perft(&mut small, 5, true, None);
        t0.board.food.sort();
        small.board.food.sort();
        assert_eq!(t0.board.food, small.board.food);
        assert_eq!(small.board.snakes, t0.board.snakes);
    }
    #[test]
    fn move_generation_food() {
        let contents =
            fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\body.json")
                .expect("Something went wrong reading the file");
        // make it into a smallrequest
        let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
        let small = seralized.into_small();
        let mut moves_you = small.snake_moves(0);
        moves_you.sort();
        let mut moves_other = small.snake_moves(1);
        moves_other.sort();
        let mut movesothercheck = array_vec![[Move;4] => Move{id : 1, direction : crate::makeunmake::Direction::Left}, Move{id : 1, direction : crate::makeunmake::Direction::Up}];
        movesothercheck.sort();
        assert_eq!(moves_other, movesothercheck);
        assert_eq!(
            moves_you,
            array_vec![[Move;4] => Move{id : 0, direction : crate::makeunmake::Direction::Left}]
        )
    }

    #[test]

    fn movegen_same_direction_test() {
        fn move_generation_food() {
            let contents =
                fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\movgen.json")
                    .expect("Something went wrong reading the file");
            // make it into a smallrequest
            let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
            let small = seralized.into_small();
            let mut moves_you = small.snake_moves(0);
            moves_you.sort();
            let mut moves_actual = array_vec![[Move;4] => Move{id : 0, direction : crate::makeunmake::Direction::Left}, Move{id : 0, direction : crate::makeunmake::Direction::Up} , Move{id : 0, direction : crate::makeunmake::Direction::Right}];
            moves_actual.sort();
            assert_eq!(moves_you, moves_actual)
        }
    }

    fn perft(state: &mut SmallRequest, depth: u8, player: bool, you_move: Option<Move>) {
        if state.amount_alive() as usize == 0
            || !state.board.snakes[state.you].alive
            || state.amount_alive() as usize == 1
            || depth == 0
        {
            return;
        }
        // println!("{:?}", state.snake_moves(state.you));
        if player {
            for x in state.snake_moves(state.you) {
                perft(state, depth, !player, Some(x));
            }
        } else {
            for x in state.all_snake_moves(you_move.unwrap()) {
                let t0 = state.clone();
                let delta = state.make_move(&x);
                perft(state, depth - 1, !player, None);
                state.unmake_move(&delta);
                assert_eq!(state.board.snakes, t0.board.snakes);
            }
        }
    }
}
