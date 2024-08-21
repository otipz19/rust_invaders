use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}, QueueableCommand};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, prev_frame: &Frame, cur_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in cur_frame.iter().enumerate() {
        for (y, cell) in col.iter().enumerate() {
            if *cell != prev_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *cell);
            }
        }
    }

    stdout.flush().unwrap();
}