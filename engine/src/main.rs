#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::net::IpAddr;
use std::str::FromStr;
use std::time::Instant;

use board::board::GameRequest;

use movegen::genmove::*;

use rocket::config::Config;
use rocket::http::Status;
use rocket::routes;
use rocket::serde::json::{json, Json, Value};
use search::search::Search;

#[get("/")]
fn handle_index() -> Value {
    json!({
        "apiversion": "1",
        "author": "BrokenKeyboard",
        "color": "#888888",
        "head": "default",
        "tail": "default",
    })
}

#[post("/start", format = "json", data = "<_start_req>")]
fn handle_start(_start_req: Json<GameRequest>) -> Status {
    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameRequest>) -> Value {
    let mut small = move_req.into_small();
    let t0 = Instant::now();
    let mut thing0 = small.clone();
    let eval = small.minimax(7, i32::MIN, i32::MAX, true, None);
    thing0.board.food.sort();
    small.board.food.sort();
    assert_eq!(thing0.board.food, small.board.food);
    assert_eq!(thing0.board.snakes, small.board.snakes);
    let t1 = Instant::now();
    let you_moves = small.snake_moves(small.you);
    if you_moves.len() == 1 {
        return json!({ "move":  you_moves[0].direction.to_string()});
    }
    println!("{:?}", small.snake_moves(small.you));
    println!("{:?}", t1 - t0);
    // assert!(t0
    // .board
    // .food
    // .iter()
    // .all(|item| small.board.food.contains(item)));
    // t0.board.food.sort();
    // small.board.food.sort();
    // pretty_assertions::assert_eq!(t0.board.food, small.board.food);
    // assert_eq!(small.board.snakes, t0.board.snakes);
    println!(
        "turn: {},score: {}, direction : {:?}",
        move_req.turn, eval.score, eval.direction
    );

    json!({ "move":  eval.direction.unwrap().to_string()})
}

#[post("/end", format = "json", data = "<_end_req>")]
fn handle_end(_end_req: Json<GameRequest>) -> Status {
    println!("end");
    Status::Ok
}

#[launch]
fn throngler() -> _ {
    let address = "0.0.0.0";
    let env_port = "8000";
    let port = env_port.parse::<u16>().unwrap();

    let mut config = Config::release_default();
    config.address = IpAddr::from_str(address).unwrap();
    config.port = port;
    rocket::build().mount(
        "/",
        routes![handle_index, handle_start, handle_move, handle_end],
    )
}
