use crate::{COLS_NUM, ROWS_NUM};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(COLS_NUM);

    for _ in 0..COLS_NUM {
        let mut col = Vec::with_capacity(ROWS_NUM);
        for _ in 0..ROWS_NUM {
            col.push(" ");
        }
        cols.push(col);
    }

    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
