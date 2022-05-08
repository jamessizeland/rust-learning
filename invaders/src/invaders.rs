use rusty_time::prelude::Timer;

use crate::{NUM_COLS, NUM_ROWS};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32, // 1 = right, 0 = left
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
                    && (y % 2 == 0) {
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
}