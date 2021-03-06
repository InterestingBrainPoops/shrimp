use std::fs;

use board::{board::GameRequest, small::SmallRequest};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use movegen::{
    genmove::GenMove,
    makeunmake::{Direction, MakeUnmake, Move},
};
// use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn perft(state: &mut SmallRequest, depth: u8, player: bool, you_move: Option<Move>) {
    if state.amount_alive() as usize == 0
        || !state.board.snakes[state.you].alive
        || state.amount_alive() as usize == 1
        || depth == 0
    {
        // im the only one left
        return;
    }
    if player {
        for x in state.snake_moves(state.you) {
            perft(state, depth, !player, Some(x));
        }
    } else {
        let mut delta;
        for x in state.all_snake_moves(you_move.unwrap()) {
            // let t0 = state.clone();
            delta = state.make_move(&x);
            perft(state, depth - 1, !player, None);
            state.unmake_move(&delta);
            // assert_eq!(state.board.snakes, t0.board.snakes);
        }
    }
}

fn movegen_perft_midgame(c: &mut Criterion) {
    let contents =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\movegen.json")
            .expect("Something went wrong reading the file");
    // make it into a smallrequest
    let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
    let mut small = seralized.into_small();
    c.bench_function("movegen_perft_midgame", |b| {
        b.iter(|| perft(&mut small, 5, true, None))
    });
}

fn movegen_perft_earlygame(c: &mut Criterion) {
    let contents =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
            .expect("Something went wrong reading the file");
    // make it into a smallrequest
    let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
    let mut small = seralized.into_small();
    c.bench_function("movegen_perft_early", |b| {
        b.iter(|| perft(&mut small, 5, true, None))
    });
}

fn move_make_midgame(c: &mut Criterion) {
    let contents =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\movegen.json")
            .expect("Something went wrong reading the file");
    // make it into a smallrequest
    let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
    let mut small = seralized.into_small();
    let move_to_make = small.all_snake_moves(Move::new(Direction::Up, 0))[0];
    c.bench_function("move_make_midgame", |b| {
        b.iter(|| {
            let delta = small.make_move(&move_to_make);
            small.unmake_move(&delta);
        })
    });
}

fn movegen_midgame(c: &mut Criterion) {
    let contents =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\movegen.json")
            .expect("Something went wrong reading the file");
    // make it into a smallrequest
    let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
    let small = seralized.into_small();
    c.bench_function("movegen_midgame", |b| {
        b.iter(|| {
            small.all_snake_moves(black_box(Move {
                id: 0,
                direction: Direction::Up,
            }))
        })
    });
}
fn make_all_moves(c: &mut Criterion) {
    let contents =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\four_player.json")
            .expect("Something went wrong reading the file");
    // make it into a smallrequest
    let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
    let mut small = seralized.into_small();
    let all_moves = small.all_snake_moves(Move::new(Direction::Down, 0));
    c.bench_function("make_all_move_4_player", |b| {
        b.iter(|| {
            for x in all_moves {
                let delta = small.make_move(&x);
                small.unmake_move(&delta);
            }
        })
    });
}
criterion_group!(
    benches,
    make_all_moves,
    movegen_perft_midgame,
    movegen_perft_earlygame,
    move_make_midgame,
    movegen_midgame,
);
criterion_main!(benches);
