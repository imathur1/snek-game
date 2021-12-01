pub type Coord = (i32, i32);

#[derive(PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

pub type SnekId = u8;

pub enum UpdateResult {
    Nothing,
    WallCollision,
    PlayerCollision(SnekId)
}