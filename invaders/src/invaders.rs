use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};
use rusty_time::prelude::Timer;
use std::{cmp::max, time::Duration};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32, // 1 = right, -1 = left
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < (NUM_ROWS/2) - 1)
                    && (x % 2 == 0) // only on evens
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            // time ellapsed
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                // get the x values of the invaders and return the minimum or 0 if no invaders left
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    // hit leftmost edge, turn back and go down one row
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self
                    .army
                    .iter()
                    .map(|invader| invader.x)
                    .max()
                    .unwrap_or(NUM_COLS - 1);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                // move down and increase speed of invaders!!!
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1; // move every invader down by 1
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
    pub fn all_killed(&self) -> bool {
        // win condition
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        // lose condition
        self.army.iter().map(|invader| invader.y).min().unwrap_or(0) >= NUM_ROWS - 1
    }
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        // attempt to kill an invader
        if let Some(index) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(index); // shot hit invader, remove it
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                // "⛩️"
                "x"
            } else {
                // "🗼"
                "+"
            };
        }
    }
}
