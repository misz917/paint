#[derive(Clone, Copy, PartialEq, Debug)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

impl<T> XY<T> {
    pub fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}

impl From<XY<usize>> for XY<f32> {
    fn from(value: XY<usize>) -> Self {
        XY {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}
