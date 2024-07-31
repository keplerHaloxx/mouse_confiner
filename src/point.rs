pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn scale_point_to_screen(&self, width: i32, height: i32) -> Self {
        Self::new(
            (65535 / (width - 1)) * (self.x),
            (65535 / (height - 1)) * (self.y),
        )
    }
}
