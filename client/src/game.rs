use std::collections::HashMap;

use macroquad::prelude::*;
use crate::snek::Snek;
use shared::{Coord, Direction, SnekId, UpdateResult, MAX_PLAYERS};


const STARTING_LENGTH: i32 = 10;

pub struct Game {
    pub screen_width: i32,
    pub screen_height: i32,
    pub grid_size: i32,
    pub grid_x_count: i32,
    pub grid_y_count: i32,
    
    grid_width: i32,
    grid_height: i32,
    grid_x: i32,
    grid_y: i32,
    internal_grid: Vec<SnekId>,
    pub sneks: HashMap<SnekId, Snek>,
    my_snek_id: SnekId,

    started: bool
}

impl Game {
    pub fn new(screen_width: i32, screen_height: i32,
        grid_size: i32, grid_x_count: i32, grid_y_count: i32) -> Game 
    {
        Game { 
            screen_width, screen_height, grid_size, grid_x_count, grid_y_count,
            grid_width: grid_size * grid_x_count, grid_height: grid_size * grid_y_count, grid_x: 50, grid_y: 50,
            internal_grid: vec![0; (grid_x_count * grid_y_count) as usize],
            sneks: HashMap::new(),
            my_snek_id: 0,
            started: false
        }
    }

    pub fn get_my_snek_id(&self) -> SnekId {
        // Get my snek id
        self.my_snek_id
    }

    pub fn set_my_snek_id(&mut self, id: SnekId) {
        // Set my snek id
        self.my_snek_id = id;
    }

    pub fn has_started(&self) -> bool {
        // Check if started
        self.started
    }

    pub fn start_game(&mut self) {
        // Start the game
        self.started = true;
    }

    pub fn end_game(&mut self) {
        // End the game
        self.started = false;
    }

    fn get_spawn(&self, id: SnekId) -> Result<(Coord, Vec<Coord>, Direction), &str> {
        // Get the spawn locations based on # of players
        match id {
            1 => Ok((
                (STARTING_LENGTH - 1, 0), 
                (0..STARTING_LENGTH - 1).into_iter().rev().map(|x| (x, 0)).collect(),
                Direction::East
            )),
            2 => Ok((
                (STARTING_LENGTH - 1, self.grid_y_count - 1), 
                (0..STARTING_LENGTH - 1).into_iter().rev().map(|x| (x, self.grid_y_count - 1)).collect(),
                Direction::East
            )),
            3 => Ok((
                (self.grid_x_count - STARTING_LENGTH, 0), 
                (self.grid_x_count - STARTING_LENGTH + 1..self.grid_x_count).into_iter().map(|x| (x, 0)).collect(),
                Direction::West
            )),
            4 => Ok((
                (self.grid_x_count - STARTING_LENGTH, self.grid_y_count - 1), 
                (self.grid_x_count - STARTING_LENGTH + 1..self.grid_x_count).into_iter().map(|x| (x, self.grid_y_count - 1)).collect(),
                Direction::West
            )),
            _ => Err("Exceeded player count!")
        }
    }

    pub fn spawn_snek(&mut self, id: SnekId) -> Result<(), &str> {
        // Spawn the snek at specified location
        if self.sneks.len() >= MAX_PLAYERS {
            return Err("Exceeded player count!");
        }

        let (head, body, direction) = self.get_spawn(id).unwrap();
        Game::set_snek_at(head.0, head.1, id, self.grid_x_count, &mut self.internal_grid);
        for coord in &body {
            Game::set_snek_at(coord.0, coord.1, id, self.grid_x_count, &mut self.internal_grid);
        }
        self.sneks.insert(id, Snek { id, head, body, previous_direction: Direction::Invalid, direction });
        Ok(())
    }

    pub fn update(&mut self, should_update: bool) {
        // Update sneks
        if self.started && should_update {
            let mut dead: Vec<SnekId> = Vec::new();
            // Check for collisions
            for (id, snek) in self.sneks.iter_mut() {
                // println!("Snek of {} is going {}", id, snek.direction as u8);
                let result = Game::update_snek(snek, &mut self.internal_grid, self.grid_x_count, self.grid_y_count);
                match result {
                    UpdateResult::WallCollision => {
                        dead.push(*id);
                    },
                    UpdateResult::PlayerCollision(_) => {
                        dead.push(*id);
                    },
                    _ => {}
                }
            }
            // Remove the dead sneks
            for id in dead {
                // println!("snek {} died!", id);
                Game::remove_snek(id, &mut self.sneks, &mut self.internal_grid, self.grid_x_count);
            }
        }
        
        // Draw the sneks
        for (_, snek) in self.sneks.iter() {
            draw_rectangle((self.grid_x + snek.head.0 * self.grid_size) as f32, 
                (self.grid_y + snek.head.1 * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, YELLOW);
            draw_rectangle_lines((self.grid_x + snek.head.0 * self.grid_size) as f32, 
            (self.grid_y + snek.head.1 * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, 2.0, BLACK);

            for (x, y) in &snek.body {
                draw_rectangle((self.grid_x + x * self.grid_size) as f32, 
                    (self.grid_y + y * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, RED);
                draw_rectangle_lines((self.grid_x + x * self.grid_size) as f32, 
                    (self.grid_y + y * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, 2.0, BLACK);
            }
        }

        // Draw the grid
        const THICKNESS: f32 = 10.0;
        draw_rectangle_lines(self.offset_x(0) as f32 - THICKNESS / 2.0, self.offset_y(0) as f32 - THICKNESS / 2.0,
            self.grid_width as f32 + THICKNESS, self.grid_height as f32 + THICKNESS, THICKNESS, GREEN);
        // for row in 0..=self.grid_row_count {
        //     let y = self.offset_y(row * self.grid_size) as f32;
        //     draw_line(self.offset_x(0) as f32, y, self.offset_y(self.grid_width) as f32, y, 1.5, GREEN);
        // }
        // for col in 0..=self.grid_col_count {
        //     let x = self.offset_x(col * self.grid_size) as f32;
        //     draw_line(x, self.offset_y(0) as f32, x, self.offset_y(self.grid_height) as f32, 1.5, GREEN);
        // }

        // if let Some(snek) = self.sneks.get_mut(&self.my_snek_id) {
        //     let snek_move;
        //     if snek.direction == Direction::North {
        //         snek_move = String::from("W");
        //     } else if snek.direction == Direction::South {
        //         snek_move = String::from("S");
        //     } else if snek.direction == Direction::West {
        //         snek_move = String::from("A");
        //     } else {
        //         snek_move = String::from("D");
        //     }
        //     socket.send(Packet::reliable_ordered(
        //         *server,
        //         snek_move.as_bytes().to_vec(),
        //         Some(5),
        //     )).unwrap();
        // } 
    }

    pub fn get_all_snek_ids(&self) -> Vec<SnekId> {
        // Get all of the snek ids
        self.sneks.keys().cloned().collect()
    }

    pub fn is_alive(&self, snek_id: SnekId) -> bool {
        // Check if a snek is still alive
        self.sneks.contains_key(&snek_id)
    }
    
    pub fn get_previous_snek_direction(&self, snek_id: SnekId) -> Direction {
        // Get the snek's previous direction
        self.sneks[&snek_id].previous_direction
    }

    pub fn set_previous_snek_direction(&mut self, snek_id: SnekId, direction: Direction) {
        // Set the snek's previous direction
        self.sneks.get_mut(&snek_id).unwrap().previous_direction = direction;
    }

    pub fn get_snek_direction(&self, snek_id: SnekId) -> Direction {
        // Get the snek's current direction
        self.sneks[&snek_id].direction
    }

    pub fn move_snek(&mut self, snek_id: SnekId, direction: Direction) {
        // Move the snek in specified direction
        if let Some(snek) = self.sneks.get_mut(&snek_id) {
            // if !snek.has_changed_direction {
            //     snek.has_changed_direction = true;
            //     snek.set_direction(direction);
            // }
            snek.set_direction(direction);
        }
    }

    pub fn handle_events(&mut self) {
        // Get arrow key input
        if !self.has_started() {
            return;
        }
        if is_key_down(KeyCode::Up) {
            self.move_snek(self.my_snek_id, Direction::North);
        } else if is_key_down(KeyCode::Down) {
            self.move_snek(self.my_snek_id, Direction::South);
        } else if is_key_down(KeyCode::Right) {
            self.move_snek(self.my_snek_id, Direction::East);
        } else if is_key_down(KeyCode::Left) {
            self.move_snek(self.my_snek_id, Direction::West);
        }
    }

    fn update_snek(snek: &mut Snek, grid: &mut Vec<SnekId>, width: i32, height: i32) -> UpdateResult {
        // snek.has_changed_direction = false;
        // Check for collisions
        let new_head = snek.get_new_head_coord();
        if new_head.0 < 0 || new_head.0 >= width || new_head.1 < 0 || new_head.1 >= height {
            return UpdateResult::WallCollision;
        }
        let snek_id = snek.id;
        match Game::get_snek_at(new_head.0, new_head.1, width, grid) {
            0 => {},
            id => {
                if id == snek_id {
                    return UpdateResult::WallCollision;
                } else {
                    return UpdateResult::PlayerCollision(id);
                }
            }
        }
        // Update the internal grid
        // Add head
        let head_index = Game::get_1d_index(new_head.0, new_head.1, width);
        grid[head_index] = snek_id;

        // Remove old tail
        let tail = snek.body.last().unwrap();
        let tail_index = Game::get_1d_index(tail.0, tail.1, width);
        grid[tail_index] = 0;

        // Advance the snek itself
        snek.advance(false);

        return UpdateResult::Nothing;
    }

    fn remove_snek(id: SnekId, sneks: &mut HashMap<SnekId, Snek>, grid: &mut Vec<SnekId>, width: i32) {
        // Remove snek from board
        let snek = sneks.remove(&id).unwrap();
        let index = Game::get_1d_index(snek.head.0, snek.head.1, width);
        grid[index] = 0;
        for coord in snek.body {
            let index = Game::get_1d_index(coord.0, coord.1, width);
            grid[index] = 0;
        }
    }

    fn offset_x(&self, x: i32) -> i32 {
        // Offset x coordinate
        self.grid_x + x
    }

    fn offset_y(&self, y: i32) -> i32 {
        // Offset y coordinate
        self.grid_y + y
    }

    fn get_1d_index(x: i32, y: i32, width: i32) -> usize {
        // Get the value at the specific location on grid
        (x + y * width) as usize
    }

    fn get_snek_at(x: i32, y: i32, width: i32, grid: &Vec<SnekId>) -> SnekId {
        // Get the snek at specified location
        grid[Game::get_1d_index(x, y, width)]
    }

    fn set_snek_at(x: i32, y: i32, id: SnekId, width: i32, grid: &mut Vec<SnekId>) {
        // Set the snek at specified location
        grid[Game::get_1d_index(x, y, width)] = id;
    }
}