// 3.1
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.top_left.y - self.bottom_right.y;
        width * height
    }
}

pub fn square(lower_left: Point, length: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: lower_left.x,
            y: lower_left.y + length,
        },
        bottom_right: Point {
            x: lower_left.x + length,
            y: lower_left.y,
        },
    }
}
