use crate::{
    common::{math::Math, xy::XY},
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
    }
}
