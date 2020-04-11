// Functions - Used to store blocks of code for re-use
// Make sure to keep 1 of the 2 sections commented out to prevent errors


fn greeting(greet: &str, name: &str) {
  println!("{} {}, you're looking well today", greet, name)
}

pub fn run() {
  greeting("Hello", "Dave");
}

// ---------------------------------------------------------

// fn add(n1: i32, n2: i32) -> i32 {
//   n1 + n2
// }

// pub fn run() {
//   // Bind function values to variables
//   let get_sum = add(5, 5);
//   println!("Sum: {}", get_sum);

//   //Closure
//   let n3: i32 = 10;
//   let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
//   println!("C Sum: {}", add_nums(3, 3));
// }
