use crate::{canvas::Canvas, common::XY};

pub trait CanvasDrawable {
    fn draw(&self, canvas: &mut Canvas) {}
}

pub struct Positions {
    p1: XY<usize>,
    p2: XY<usize>,
}

pub struct Square {
    positions: Positions,
}

pub struct Circle {
    positions: Positions,
}

pub struct Line {
    positions: Positions,
}
