pub fn run() {
  let age: u8 = 21;
  let check_id: bool = true;

  // If/Else
  if age >= 21 && check_id {
    println!("Bartender: What u want to drink?");
  } else if age < 21 && check_id {
    println!("Bartender: Sorry, but u've to leave!");
  } else {
    println!("Bartender: Show ur ID!");
  }

  // Short If
  let is_of_age = if age >= 21 { true } else { false };
  println!("Is of age: {:?}", is_of_age);
}
