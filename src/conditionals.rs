// Conditionals - Used to check the condition of something and act on it

pub fn run() {
  let age: u8 = 21;
  let check_id: bool = false;

  // If/Else
  if check_id && age >= 21 {
    println!("Bartender: What would you like to drink?")
  } else if check_id && age < 21{
    println!("Bartender: Sorry, you have to leave.")
  } else {
    println!("Bartender: Can I see some ID?")
  }

  // Shorthand If
  let is_of_age = if age >= 21 { true } else  { false };
  println!("Is Of Age: {}", is_of_age);
}