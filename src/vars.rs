// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Marc";
  let mut age = 36;

  age = 37;

  println!("My name is {} and I am {}", name, age);
}
