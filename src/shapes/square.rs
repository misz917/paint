use crate::{
    canvas::Canvas,
    common::{CanvasDrawable, xy::XY},
    pencil::Pencil,
    shapes::Common,
};

pub struct Square {
    common: Common,
}

impl CanvasDrawable for Square {
    fn draw(&self, canvas: &mut Canvas) {
        let top_left = self.common.p1;
        let bottom_right = self.common.p2;
        let top_right = XY::new(bottom_right.x, top_left.y);
        let bottom_left = XY::new(top_left.x, bottom_right.y);

        let corners = [top_left, top_right, bottom_right, bottom_left];

        for i in 0..4 {
            Pencil::draw_line(
                canvas,
                corners[i],
                corners[(i + 1) % 4],
                self.common.line_size,
                self.common.color,
            );
        }
    }
}

impl Square {
    pub fn new(p1: XY<usize>, p2: XY<usize>, line_size: usize, color: u32) -> Self {
        Square {
            common: Common {
                p1,
                p2,
                line_size,
                color,
            },
        }
    }
}
