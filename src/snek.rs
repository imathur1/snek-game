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
            Direction::North => (self.head.0, self.head.1 - 1),
            Direction::South => (self.head.0, self.head.1 + 1),
            Direction::East => (self.head.0 + 1, self.head.1),
            Direction::West => (self.head.0 - 1, self.head.1),
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        // Prohibit moving in the opposite direction
        match direction {
            Direction::North => if self.direction == Direction::South { return; },
            Direction::South => if self.direction == Direction::North { return; },
            Direction::East => if self.direction == Direction::West { return; },
            Direction::West => if self.direction == Direction::East { return; }
        };
        self.direction = direction;
    }
    
    pub fn advance(&mut self, should_grow: bool) {
        if !should_grow {
            self.body.pop();
        }
        self.body.insert(0, self.head);
        self.head = self.get_new_head_coord();
    }
}