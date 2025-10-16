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

impl<T> Into<f32> for XY<T>
where
    T: Into<f32> + Copy,
{
    fn into(self) -> f32 {
        let x_f32: f32 = self.x.into();
        let y_f32: f32 = self.y.into();
        (x_f32.powi(2) + y_f32.powi(2)).sqrt()
    }
}
