use crate::common::XY;

pub trait Shape {}

pub struct TwoPoints {
    p1: XY<usize>,
    p2: XY<usize>,
}

pub struct Line {
    points: TwoPoints,
}
impl Shape for Line {}

pub struct Square {
    points: TwoPoints,
}
impl Shape for Square {}

pub struct Circle {
    points: TwoPoints,
}
impl Shape for Circle {}

// points describe the shape and the position
