use std::time::Duration;

use crate::{ frame::{ Drawable, Frame }, invaders::Invaders, shot::Shot, COLS_NUM, ROWS_NUM };

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: COLS_NUM / 2,
            y: ROWS_NUM - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < COLS_NUM - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.is_dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut is_hit = false;

        for shot in self.shots.iter_mut() {
            if !shot.is_exploding && invaders.kill_invader_at(shot.x, shot.y) {
                shot.explode();
                is_hit = true;
            }
        }

        is_hit
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
