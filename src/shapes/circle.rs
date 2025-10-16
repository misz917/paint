use crate::shapes::{CanvasDrawable, Common};

pub struct Circle {
    common: Common,
}

impl CanvasDrawable for Circle {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        // let circumference = self.common.p2 - self.common.p1
    }
}
