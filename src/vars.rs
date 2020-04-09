// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Marc";
  let mut age = 36;
  // Read age = 36
  println!("My name is {} and I am {}", name, age);
  // Read age = 37
  age = 37;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( my_name, my_age ) = ("Marc", 36);
  println!("{} is {}", my_name, my_age );
}
