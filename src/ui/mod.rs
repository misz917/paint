use crate::common::xy::XY;

pub mod body_builder;
pub mod button;
pub mod text_field;

struct Common {
    location: XY<usize>,
    body: Vec<u32>, // also known as buffer
}
