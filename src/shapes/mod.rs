use crate::common::xy::XY;

pub mod circle;
pub mod line;
pub mod square;

struct Common {
    p1: XY<usize>,
    p2: XY<usize>,
    line_size: usize,
    color: u32,
}
