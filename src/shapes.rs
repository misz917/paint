use std::io::LineWriter;

use crate::{common::XY, ui::CanvasDrawable};

pub struct TwoPoints {
    p1: XY<usize>,
    p2: XY<usize>,
}

pub struct Line {
    points: TwoPoints,
}

pub struct Square {
    points: TwoPoints,
}

pub struct Circle {
    points: TwoPoints,
}

impl CanvasDrawable for Line {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        todo!()
    }
}

// points describe the shape and the position
