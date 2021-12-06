use core::time;
use std::{collections::HashMap, convert::TryInto};
use laminar::{ErrorKind, Packet, Socket, SocketEvent};

use macroquad::prelude::*;
use crate::snek::Snek;
use crate::shared::{Coord, Direction, SnekId, UpdateResult};

const MAX_PLAYERS: usize = 4;
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
    current_snek_id: SnekId,

    last_time: f64
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
            current_snek_id: 0,
            last_time: get_time()
        }
    }

    fn select_spawn(&self) -> Result<(Coord, Vec<Coord>, Direction), &str> {
        match self.sneks.len() {
            0 => Ok((
                (STARTING_LENGTH - 1, 0), 
                (0..STARTING_LENGTH - 1).into_iter().rev().map(|x| (x, 0)).collect(),
                Direction::East
            )),
            1 => Ok((
                (STARTING_LENGTH - 1, self.grid_y_count - 1), 
                (0..STARTING_LENGTH - 1).into_iter().rev().map(|x| (x, self.grid_y_count - 1)).collect(),
                Direction::East
            )),
            2 => Ok((
                (self.grid_x_count - STARTING_LENGTH, 0), 
                (self.grid_x_count - STARTING_LENGTH + 1..self.grid_x_count).into_iter().map(|x| (x, 0)).collect(),
                Direction::West
            )),
            _ => Err("Exceeded player count!")
        }
    }

    pub fn spawn_snek(&mut self, is_me: bool) -> Result<SnekId, &str> {
        if self.sneks.len() >= MAX_PLAYERS {
            return Err("Exceeded player count!");
        }
        self.current_snek_id += 1;
        let id = self.current_snek_id;

        let (head, body, direction) = self.select_spawn().unwrap();
        Game::set_snek_at(head.0, head.1, id, self.grid_x_count, &mut self.internal_grid);
        for coord in &body {
            Game::set_snek_at(coord.0, coord.1, id, self.grid_x_count, &mut self.internal_grid);
        }
        self.sneks.insert(id, Snek { id, head, body, direction, has_changed_direction: false });

        if is_me {
            self.my_snek_id = id;
        }
        Ok(id)
    }

    pub fn update(&mut self, socket: &mut Socket, server: &std::net::SocketAddr) {
        // Update sneks
        let time_passed: bool = (get_time() - self.last_time) >= 0.15;
        if time_passed {
            let mut dead: Vec<SnekId> = Vec::new();
            for (id, snek) in self.sneks.iter_mut() {
                let result = Game::update_snek(snek, &mut self.internal_grid, self.grid_x_count, self.grid_y_count);
                match result {
                    UpdateResult::WallCollision => {
                        dead.push(*id);
                    },
                    UpdateResult::PlayerCollision(snek_id) => {
                        dead.push(*id);
                    },
                    _ => {}
                }
            }
            for id in dead {
                Game::remove_snek(id, &mut self.sneks, &mut self.internal_grid, self.grid_x_count);
            }

            self.last_time = get_time();
        }
        
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

        // Read input
        self.handle_events();  

        if let Some(snek) = self.sneks.get_mut(&self.my_snek_id) {
            let mut snek_move = String::new();
            if (snek.direction == Direction::North) {
                snek_move = String::from("W");
            } else if (snek.direction == Direction::South) {
                snek_move = String::from("S");
            } else if (snek.direction == Direction::West) {
                snek_move = String::from("A");
            } else {
                snek_move = String::from("D");
            }
            socket.send(Packet::reliable_ordered(
                *server,
                snek_move.as_bytes().to_vec(),
                Some(7),
            ));
        } 
    }

    pub fn handle_events(&mut self) {
        if let Some(snek) = self.sneks.get_mut(&self.my_snek_id) {
            if !snek.has_changed_direction {
                if is_key_pressed(KeyCode::Up) {
                    snek.has_changed_direction = true;
                    snek.set_direction(Direction::North);
                } else if is_key_pressed(KeyCode::Down) {
                    snek.has_changed_direction = true;
                    snek.set_direction(Direction::South);
                } else if is_key_pressed(KeyCode::Right) {
                    snek.has_changed_direction = true;
                    snek.set_direction(Direction::East);
                } else if is_key_pressed(KeyCode::Left) {
                    snek.has_changed_direction = true;
                    snek.set_direction(Direction::West);
                }
            }
        }
        // if let Some(snek) = self.sneks.get_mut(&2) {
        //     if !snek.has_changed_direction {
        //         if is_key_pressed(KeyCode::W) {
        //             snek.has_changed_direction = true;
        //             snek.set_direction(Direction::North);
        //         } else if is_key_pressed(KeyCode::S) {
        //             snek.has_changed_direction = true;
        //             snek.set_direction(Direction::South);
        //         } else if is_key_pressed(KeyCode::D) {
        //             snek.has_changed_direction = true;
        //             snek.set_direction(Direction::East);
        //         } else if is_key_pressed(KeyCode::A) {
        //             snek.has_changed_direction = true;
        //             snek.set_direction(Direction::West);
        //         }
        //     }
        // }
    }

    fn update_snek(snek: &mut Snek, grid: &mut Vec<SnekId>, width: i32, height: i32) -> UpdateResult {
        snek.has_changed_direction = false;

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
        let snek = sneks.remove(&id).unwrap();
        let index = Game::get_1d_index(snek.head.0, snek.head.1, width);
        grid[index] = 0;
        for coord in snek.body {
            let index = Game::get_1d_index(coord.0, coord.1, width);
            grid[index] = 0;
        }
    }

    fn offset_x(&self, x: i32) -> i32 {
        self.grid_x + x
    }

    fn offset_y(&self, y: i32) -> i32 {
        self.grid_y + y
    }

    fn get_1d_index(x: i32, y: i32, width: i32) -> usize {
        (x + y * width) as usize
    }

    fn get_snek_at(x: i32, y: i32, width: i32, grid: &Vec<SnekId>) -> SnekId {
        grid[Game::get_1d_index(x, y, width)]
    }

    fn set_snek_at(x: i32, y: i32, id: SnekId, width: i32, grid: &mut Vec<SnekId>) {
        grid[Game::get_1d_index(x, y, width)] = id;
    }
}