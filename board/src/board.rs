pub struct GameRequest {
    game: GameInfo,
    turn: i64,
    board: BoardInfo,
    you: BattleSnake,
}

struct GameInfo {
    id: String,
    ruleset: Ruleset,
    timeout: i64,
}

struct Ruleset {
    name: String,
    version: String,
}

struct BoardInfo {
    height: u8,
    width: u8,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<BattleSnake>,
}

struct BattleSnake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: u16,
    shout: String,
    squad: String,
}

pub struct Coordinate {
    x: i32,
    y: i32,
}
