use std::{collections::HashMap, convert::TryInto};

use macroquad::prelude::*;
use crate::snek::Snek;

pub struct Game {
    pub screen_width: i32,
    pub screen_height: i32,
    pub grid_size: i32,
    pub grid_col_count: i32,
    pub grid_row_count: i32,
    
    grid_width: i32,
    grid_height: i32,
    grid_x: i32,
    grid_y: i32,
    internal_grid: Vec<u8>,
    snek_ids: Vec<u8>,
    sneks: HashMap<u8, Snek>
}

impl Game {
    pub fn new(screen_width: i32, screen_height: i32,
        grid_size: i32, grid_col_count: i32, grid_row_count: i32) -> Game 
    {
        Game { 
            screen_width, screen_height, grid_size, grid_col_count, grid_row_count,
            grid_width: grid_size * grid_col_count, grid_height: grid_size * grid_row_count, grid_x: 50, grid_y: 50,
            internal_grid: Vec::with_capacity((grid_col_count * grid_row_count).try_into().unwrap()),
            snek_ids: Vec::new(),
            sneks: HashMap::new() 
        }
    }

    pub fn add_snek(&mut self) {
        self.snek_ids.push(0);
        self.sneks.insert(0, Snek { id: 0, head: (0, 0), body: vec![(1, 0), (2, 0)]});
    }

    pub fn update(&mut self) {
        // Update sneks
        for id in &self.snek_ids {
            let snek = &mut *self.sneks.get_mut(id).unwrap();
            snek.update();

            draw_rectangle((self.grid_x + snek.head.0 * self.grid_size) as f32, 
                (self.grid_y + snek.head.1 * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, YELLOW);
            for (row, col) in &snek.body {
                draw_rectangle((self.grid_x + col * self.grid_size) as f32, 
                (self.grid_y + row * self.grid_size) as f32, self.grid_size as f32, self.grid_size as f32, RED);
            }
        }

        // Draw the grid
        for row in 0..=self.grid_row_count {
            let y = self.offset_y(row * self.grid_size) as f32;
            draw_line(self.offset_x(0) as f32, y, self.offset_y(self.grid_width) as f32, y, 1.5, GREEN);
        }
        for col in 0..=self.grid_col_count {
            let x = self.offset_x(col * self.grid_size) as f32;
            draw_line(x, self.offset_y(0) as f32, x, self.offset_y(self.grid_height) as f32, 1.5, GREEN);
        }
    }

    pub fn handle_events(&mut self) {

    }

    fn offset_x(&self, x: i32) -> i32 {
        self.grid_x + x
    }

    fn offset_y(&self, y: i32) -> i32 {
        self.grid_y + y
    }
}