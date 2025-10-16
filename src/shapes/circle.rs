use std::f32::consts::PI;

use crate::{
    common::{math::Math, xy::XY},
    pencil::Pencil,
    shapes::{CanvasDrawable, Common},
};

pub struct Circle {
    common: Common,
}

impl CanvasDrawable for Circle {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        let p1: XY<f32> = self.common.p1.try_into().unwrap();
        let p2: XY<f32> = self.common.p2.try_into().unwrap();

        let radius = Math::distance_between_points(p1, p2);

        let mut prev_x = p1.x + radius;
        let mut prev_y = p1.y;

        for deg in 1..361 {
            let x = p1.x + radius * (deg as f32 * PI / 180.0).cos();
            let y = p1.y + radius * (deg as f32 * PI / 180.0).sin();

            if x < 0.0 || y < 0.0 {
                prev_x = x;
                prev_y = y;
                continue;
            }

            Pencil::draw_line(
                canvas,
                XY::new(prev_x as usize, prev_y as usize),
                XY::new(x as usize, y as usize),
                self.common.line_size,
                self.common.color,
            );

            prev_x = x;
            prev_y = y;
        }
    }
}

impl Circle {
    pub fn new(p1: XY<usize>, p2: XY<usize>, line_size: usize, color: u32) -> Self {
        Circle {
            common: Common {
                p1,
                p2,
                line_size,
                color,
            },
        }
    }
}
