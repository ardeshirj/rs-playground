mod hello_world;
mod primitives;

fn main() {
   // run_hello_word();
   // run_primitives();
}

#[allow(dead_code)]
fn run_hello_word() {
   hello_world::james_bond();
   hello_world::print_pi();

   let complex = hello_world::Complex {
      real: 3.3,
      imag: 7.2,
   };

   println!("{}", complex);
   println!("{:?}", complex);

   let v = hello_world::List::new(vec![1, 2, 3]);
   println!("{}", v);

   let colors = [
      hello_world::Color {
         red: 128,
         green: 255,
         blue: 90,
      },
      hello_world::Color {
         red: 0,
         green: 3,
         blue: 254,
      },
      hello_world::Color {
         red: 0,
         green: 0,
         blue: 0,
      },
   ];

   for color in colors.iter() {
      println!("{}", color)
   }
}

#[allow(dead_code)]
fn run_primitives() {
   let matrix = primitives::Matrix::new(1.1, 1.2, 2.1, 2.2);
   println!("{}", matrix)
}
