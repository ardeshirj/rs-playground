use std::fmt;

// 1.2
pub fn james_bond() {
    println!("My name is {0}, {1} {0}", "Bond", "James");
}

// 1.2
pub fn print_pi() {
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}

// 1.2.2
#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {}, {}i", self.real, self.imag)
    }
}

// 1.2.2.1
pub struct List(Vec<i32>);

impl List {
    pub fn new(vec: Vec<i32>) -> List {
        List(vec)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

// 1.2.3
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({red}, {green}, {blue}) 0X{red:02X}{blue:02X}{green:02X}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}
