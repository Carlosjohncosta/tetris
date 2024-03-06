use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[macro_export]
macro_rules! point_vec {
    [$(($x:expr, $y:expr)),*] => {
        vec![
            $(
                Point::new($x, $y),
            )*
        ]
    }
}
