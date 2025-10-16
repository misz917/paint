use crate::common::{CanvasDrawable, xy::XY};

pub enum Task {
    RedrawCanvasObjects,
    SummonCanvasObject(Box<dyn CanvasDrawable>),
    LeftMouse(XY<usize>),
}
