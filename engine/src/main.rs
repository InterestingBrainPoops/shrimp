#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use board::board::GameRequest;
use board::small::SmallRequest;
use movegen::genmove::*;
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use search::search::Search;
#[get("/")]
fn handle_index() -> JsonValue {
    return json!({
        "apiversion": "1",
        "author": "BrokenKeyboard",
        "color": "#888888",
        "head": "default",
        "tail": "default",
    });
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameRequest>) -> Status {
    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameRequest>) -> JsonValue {
    // println!("e");
    let mut small = move_req.into_small();
    let t0 = small.clone();
    println!("{:?}", t0.snake_moves(t0.you));
    let eval = small.minimax(1, i32::MIN, i32::MAX, true, None);
    assert!(small
        .board
        .food
        .iter()
        .all(|item| t0.board.food.contains(item)));
    assert_eq!(small.board.snakes, t0.board.snakes);
    println!(
        "turn: {},score: {}, direction : {:?}",
        move_req.turn, eval.score, eval.direction
    );
    return json!({ "move":  eval.direction.unwrap().to_string()});
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameRequest>) -> Status {
    println!("end");
    Status::Ok
}

fn main() {
    let address = "0.0.0.0";
    let env_port = "8000";
    let port = env_port.parse::<u16>().unwrap();

    let config = Config::build(Environment::Development)
        .address(address)
        .port(port)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
        .launch();
}
