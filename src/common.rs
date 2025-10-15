#[derive(Clone, Copy, PartialEq)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

impl<T> XY<T> {
    pub fn new(x: T, y: T) -> Self {
        XY { x, y }
    }
}
