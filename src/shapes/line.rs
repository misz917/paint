use crate::{
    canvas::Canvas,
    common::xy::XY,
    pencil::Pencil,
    shapes::{CanvasDrawable, Common},
};

pub struct Line {
    common: Common,
}

impl CanvasDrawable for Line {
    fn draw(&self, canvas: &mut Canvas) {
        Pencil::draw_line(
            canvas,
            self.common.p1,
            self.common.p2,
            self.common.line_size,
            self.common.color,
        );
    }
}

impl Line {
    pub fn new(p1: XY<usize>, p2: XY<usize>, line_size: usize, color: u32) -> Self {
        Line {
            common: Common {
                p1,
                p2,
                line_size,
                color,
            },
        }
    }
}
