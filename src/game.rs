use core::time;
use std::{collections::HashMap, convert::TryInto};

use macroquad::prelude::*;
use crate::snek::Snek;
use crate::shared::{Coord, Direction, SnekId};

const MAX_PLAYERS: usize = 4;
const STARTING_LENGTH: i32 = 5;

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
    snek_ids: Vec<SnekId>,
    sneks: HashMap<SnekId, Snek>,
    my_snek_id: SnekId,

    last_time: f64
}

impl Game {
    pub fn new(screen_width: i32, screen_height: i32,
        grid_size: i32, grid_x_count: i32, grid_y_count: i32) -> Game 
    {
        Game { 
            screen_width, screen_height, grid_size, grid_x_count, grid_y_count,
            grid_width: grid_size * grid_x_count, grid_height: grid_size * grid_y_count, grid_x: 50, grid_y: 50,
            internal_grid: Vec::with_capacity((grid_x_count * grid_y_count).try_into().unwrap()),
            snek_ids: Vec::new(),
            sneks: HashMap::new(),
            my_snek_id: 0,
            last_time: get_time()
        }
    }

    fn select_spawn(&self) -> Result<(Coord, Vec<Coord>, Direction), &str> {
        println!("{}", self.snek_ids.len());
        match self.snek_ids.len() {
            0 => Ok((
                (STARTING_LENGTH - 1, 0), 
                (0..STARTING_LENGTH - 1).into_iter().rev().map(|x| (x, 0)).collect(),
                Direction::EAST
            )),
            1 => Ok((
                (STARTING_LENGTH - 1, self.grid_y_count - 1), 
                (0..STARTING_LENGTH - 1).into_iter().rev().map(|x| (x, self.grid_y_count - 1)).collect(),
                Direction::EAST
            )),
            2 => Ok((
                (self.grid_x_count - STARTING_LENGTH, 0), 
                (self.grid_x_count - STARTING_LENGTH + 1..self.grid_x_count).into_iter().map(|x| (x, 0)).collect(),
                Direction::WEST
            )),
            3 => Ok((
                (self.grid_x_count - STARTING_LENGTH, self.grid_y_count - 1), 
                (self.grid_x_count - STARTING_LENGTH + 1..self.grid_x_count).into_iter().map(|x| (x, self.grid_y_count - 1)).collect(),
                Direction::WEST
            )),
            _ => Err("Exceeded player count!")
        }
    }

    pub fn spawn_snek(&mut self, id: SnekId, is_me: bool) {
        if self.snek_ids.len() >= MAX_PLAYERS {
            return;
        }
        let (head, body, direction) = self.select_spawn().unwrap();
        self.sneks.insert(id, Snek { id, head, body, direction });
        self.snek_ids.push(id);
        if is_me {
            self.my_snek_id = id;
        }
    }

    pub fn update(&mut self) {
        // Update sneks
        let time_passed: bool = (get_time() - self.last_time) >= 0.2;
        if time_passed {
            self.last_time = get_time();
        }

        for id in &self.snek_ids {
            let snek = &mut *self.sneks.get_mut(id).unwrap();

            if time_passed {
                snek.update();
            }

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
    }

    pub fn handle_events(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.sneks.get_mut(&self.my_snek_id).unwrap().set_direction(Direction::NORTH);
        } else if is_key_pressed(KeyCode::Down) {
            self.sneks.get_mut(&self.my_snek_id).unwrap().set_direction(Direction::SOUTH);
        } else if is_key_pressed(KeyCode::Right) {
            self.sneks.get_mut(&self.my_snek_id).unwrap().set_direction(Direction::EAST);
        } else if is_key_pressed(KeyCode::Left) {
            self.sneks.get_mut(&self.my_snek_id).unwrap().set_direction(Direction::WEST);
        }
    }

    fn check_collisions() {

    }

    fn offset_x(&self, x: i32) -> i32 {
        self.grid_x + x
    }

    fn offset_y(&self, y: i32) -> i32 {
        self.grid_y + y
    }

    fn get_snek_at(&self, x: i32, y: i32) -> SnekId {
        self.internal_grid[(x + y * self.grid_x_count) as usize]
    }

    fn set_snek_at(&mut self, x: i32, y: i32, id: SnekId) {
        self.internal_grid[(x + y * self.grid_x_count) as usize] = id;
    }
}