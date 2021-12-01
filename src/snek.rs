use crate::shared::{Coord, Direction, SnekId};

pub struct Snek {
    pub id: SnekId,
    pub head: Coord,
    pub body: Vec<Coord>,
    pub direction: Direction,
    pub has_changed_direction: bool // reset every frame
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
        // Prohibit moving in the opposite direction
        match direction {
            Direction::NORTH => if self.direction == Direction::SOUTH { return; },
            Direction::SOUTH => if self.direction == Direction::NORTH { return; },
            Direction::EAST => if self.direction == Direction::WEST { return; },
            Direction::WEST => if self.direction == Direction::EAST { return; }
        };
        self.direction = direction;
    }
    
    pub fn advance(&mut self) {
        self.body.pop();
        self.body.insert(0, self.head);
        self.head = self.get_new_head_coord();
    }
}