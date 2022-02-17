use std::fs;

use board::{board::GameRequest, small::SmallRequest};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use movegen::{
    genmove::GenMove,
    makeunmake::{MakeUnmake, Move},
};
// use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn perft(state: &mut SmallRequest, depth: u8, player: bool, you_move: Option<Move>) {
    // println!("e");
    if state.amount_alive() as usize == 0
        || !state.board.snakes[state.you].alive
        || state.amount_alive() as usize == 1
        || depth == 0
    {
        // im the only one left
        // println!("fg");
        return;
    }
    // println!("{:?}", state.snake_moves(state.you));
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

fn criterion_benchmark(c: &mut Criterion) {
    let contents =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "\\tests\\food.json")
            .expect("Something went wrong reading the file");
    // make it into a smallrequest
    let seralized: GameRequest = serde_json::from_str(&contents).expect("Invalid json");
    let mut small = seralized.into_small();
    c.bench_function("perft5food", |b| {
        b.iter(|| perft(&mut small, 5, true, None))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
