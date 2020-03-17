use std::fmt;

#[derive(Debug)]
pub struct Matrix(f32, f32, f32, f32);

impl Matrix {
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32) -> Matrix {
        Matrix(x1, x2, y1, y2)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}
