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

        for deg in 0..360 {
            let x = p1.x + radius * (deg as f32 * PI / 180.0).cos();
            let y = p1.y + radius * (deg as f32 * PI / 180.0).sin();
            Pencil::draw_dot(
                canvas,
                XY::new(x as usize, y as usize),
                self.common.line_size,
                self.common.color,
            );
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
