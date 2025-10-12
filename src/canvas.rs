pub struct Canvas(Vec<u32>);

impl Canvas {
    pub fn new(size: usize) -> Self {
        Canvas(vec![0; size]) // 0 is color black
    }
}

impl Into<Vec<u32>> for Canvas {
    fn into(self) -> Vec<u32> {
        self.0
    }
}
