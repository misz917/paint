use crate::common::xy::XY;

pub struct Math;

impl Math {
    pub fn distance_between_points<T>(p1: XY<T>, p2: XY<T>) -> f32
    where
        T: Into<f32>,
    {
        let a: f32 = p2.x.into() - p1.x.into();
        let b: f32 = p2.y.into() - p1.y.into();
        let c = (a.powf(2.0) + b.powf(2.0)).sqrt();
        c
    }
}
