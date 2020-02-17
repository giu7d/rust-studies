pub fn run() {
  // Print
  println!("Hello, world! From print.rs!");
  // Basic formatting
  println!("{} is {} years old", "Giu", 23);
  // Positional argument
  println!("{0} is {1} years old and like {2}", "Giu", 23, "Javascript");
  // Named argument
  println!(
    "My name is {surname}, {name} {surname}",
    name = "James",
    surname = "Bond"
  );
  // Placeholder traits
  println!("Binary: {0:b}\nHex: {0:x}\nOctal: {0:o}", 10);
  // Placeholder for debug traits
  println!("{:?}", (12, true, "Hello", 0.987));
  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}
