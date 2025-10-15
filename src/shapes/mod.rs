use crate::{canvas::Canvas, common::XY};

pub mod circle;
pub mod line;
pub mod square;

struct Common {
    p1: XY<usize>,
    p2: XY<usize>,
    line_size: usize,
    color: u32,
}

pub trait CanvasDrawable {
    fn draw(&self, canvas: &mut Canvas);
}
