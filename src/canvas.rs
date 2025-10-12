use std::ops::{Index, IndexMut};

use crate::common::XY;

pub struct Canvas {
    buffer: Vec<u32>,
    dimensions: XY<usize>,
}

impl Canvas {
    pub fn new(x: usize, y: usize) -> Self {
        Canvas {
            buffer: vec![0; x * y], // 0 is color black
            dimensions: XY { x, y },
        }
    }

    /// returns reference to buffer
    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    /// returns clone of dimensions
    pub fn get_dimensions(&self) -> XY<usize> {
        self.dimensions.clone()
    }

    pub fn get_x(&self) -> usize {
        self.dimensions.x
    }

    pub fn get_y(&self) -> usize {
        self.dimensions.y
    }
}

impl Index<(usize, usize)> for Canvas {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &u32 {
        &self.buffer[y * self.dimensions.x + x]
    }
}

impl IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut u32 {
        &mut self.buffer[y * self.dimensions.x + x]
    }
}
