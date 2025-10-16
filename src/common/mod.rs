use crate::canvas::Canvas;

pub mod math;
pub mod xy;

pub trait CanvasDrawable {
    fn draw(&self, canvas: &mut Canvas);
}
