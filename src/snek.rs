use crate::shared::{Coord, Direction, SnekId};

pub struct Snek {
    pub id: SnekId,
    pub head: Coord,
    pub body: Vec<Coord>,
    pub direction: Direction
}

impl Snek {

    pub fn get_new_head_coord(&self) -> Coord {
        match self.direction {
            Direction::NORTH => (self.head.0, self.head.1 - 1),
            Direction::SOUTH => (self.head.0, self.head.1 + 1),
            Direction::EAST => (self.head.0 + 1, self.head.1),
            Direction::WEST => (self.head.0 - 1, self.head.1),
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match direction {
            Direction::NORTH => {
                if self.direction == Direction::SOUTH {
                    return;
                }
            },
            Direction::SOUTH => {
                if self.direction == Direction::NORTH {
                    return;
                }
            },
            Direction::EAST => {
                if self.direction == Direction::WEST {
                    return;
                }
            },
            Direction::WEST => {
                if self.direction == Direction::EAST {
                    return;
                }
            }
        };
        self.direction = direction;
    }
    
    pub fn update(&mut self) {
        let body_length = self.body.len();
        let mut new_body: Vec<Coord> = Vec::with_capacity(body_length);
        let mut previous_coord = self.head;
        for coord in &self.body {
            new_body.push(previous_coord);
            previous_coord = *coord;
        }
        self.head = self.get_new_head_coord();
        self.body = new_body;
    }
}