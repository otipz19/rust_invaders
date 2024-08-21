use std::{ cmp::max, time::Duration };

use rusty_time::timer::Timer;

use crate::{ frame::Drawable, COLS_NUM, ROWS_NUM };

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    direction: i32,
    move_timer: Timer,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();

        for x in 0..COLS_NUM {
            for y in 0..ROWS_NUM {
                if
                    x > 0 &&
                    x < COLS_NUM - 2 &&
                    y > 0 &&
                    y < ROWS_NUM / 2 - 1 &&
                    x % 2 == 0 &&
                    y % 2 == 0
                {
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            direction: 1,
            move_timer: Timer::from_millis(2000),
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();

            let mut should_move_downwards = false;
            if self.direction == -1 {
                let min_x = self.army
                    .iter()
                    .map(|invader| invader.x)
                    .min()
                    .unwrap_or(0);

                if min_x == 0 {
                    self.direction = 1;
                    should_move_downwards = true;
                }
            } else {
                let max_x = self.army
                    .iter()
                    .map(|invader| invader.x)
                    .max()
                    .unwrap_or(0);

                if max_x == COLS_NUM - 1 {
                    self.direction = -1;
                    should_move_downwards = true;
                }
            }

            if should_move_downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);

                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            return true;
        }

        return false;
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if
                self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32() > 0.5
            {
                "x"
            } else {
                "+"
            };
        }
    }
}
