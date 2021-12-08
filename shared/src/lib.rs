pub type Coord = (i32, i32);

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Invalid = 0,
    North = 1,
    South = 2,
    East = 3,
    West = 4
}

pub type SnekId = u8;

pub enum UpdateResult {
    Nothing,
    WallCollision,
    PlayerCollision(SnekId)
}

// https://amethyst.github.io/laminar/docs/reliability/ordering.html
#[derive(Copy, Clone)]
pub enum StreamId {
    Heartbeat = 0,
    Event = 1,
    Move = 2
}

#[derive(Copy, Clone)]
pub enum GameResult {
    Win = 0,
    Tie = 1,
    Loss = 2
}

// All packets are prepended by [magic_byte, message_type]
#[derive(Copy, Clone, PartialEq)]
pub enum MessageType {
    JoinEvent = 0,         // []
    AssignIdEvent = 1,     // [assigned_id]
    BroadcastIdsEvent = 2, // [id_1, id_2, ...]
    StartEvent = 3,        // []
    MoveEvent = 4,         // server: [id, move], client: [id_1, move_1, id_2, move_2, ...]
    DeathEvent = 5,        // [id_alive_1, id_alive_2, ...]
    EndEvent = 6,          // [result: GameResult, id_winner]
    Heartbeat = 7          // []
}

pub const MAX_PLAYERS: usize = 2;
pub const INVALID_ID: SnekId = 0;
pub const MAGIC_BYTE: u8 = 42;