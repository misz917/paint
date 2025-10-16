use crate::{
    common::{CanvasDrawable, xy::XY},
    ui::Common,
};

pub struct Button {
    common: Common,
}

impl CanvasDrawable for Button {
    fn draw(&self, canvas: &mut crate::canvas::Canvas) {
        todo!()
    }
}

impl Button {
    pub fn new() -> Self {
        todo!()
    }
}
