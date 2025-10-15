use crate::{canvas::Canvas, common::XY};

pub struct DrawableElement {
    position: XY<usize>,
    body: Vec<u32>,
}

pub trait CanvasDrawable {
    fn draw(&self, canvas: &mut Canvas);
}

pub struct Button {}
impl Button {
    pub fn press() {}
}

pub struct TextField {
    is_selected: bool,
    buffer: Vec<char>,
}
impl TextField {
    pub fn select() {}
    pub fn deselect() {}
    pub fn type_in() {}
}

impl CanvasDrawable for Button {
    fn draw(&self, canvas: &mut Canvas) {
        todo!()
    }
}

impl CanvasDrawable for TextField {
    fn draw(&self, canvas: &mut Canvas) {
        todo!()
    }
}
