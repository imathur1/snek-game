pub type Coord = (i32, i32);

#[derive(PartialEq)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

pub type SnekId = u8;