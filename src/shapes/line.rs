use crate::{
    canvas::Canvas,
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
