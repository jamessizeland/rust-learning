use crate::invaders::Invaders;
use crate::shot::Shot;
use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    // make a new player
    pub fn new() -> Self {
        Self {
            // return a player
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            // boundary check
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            // boundary check
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            // spawn new shot just above the player
            self.shots.push(Shot::new(self.x, self.y - 5));
            true // successfully shot
        } else {
            false // could not shoot
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        // did we hit something?
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                // exploding shot doesn't get to hit another invader
                if invaders.kill_invader_at(shot.x, shot.y) {
                    // check if our shot collides with invader
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
