/*
* String Types
* - str (Primitive): Immutable, fixed-lenght in memory;
* - String: Growable, heap-allocated data stucture.
*
*/

pub fn run() {
  let hello = "Hello"; // :str type, fixed-lenght
  let mut world = String::from("World"); // String, growable

  println!("{:#?}", (hello.len(), world.len()));

  // To use it we need to create a mutable variable
  world.push_str(" -Mutable- ");
  world.push('\u{1F600}');
  world.insert_str(0, "brave new ");

  // Capacity
  println!("{}", world.capacity());

  println!("{:#?}", (hello, world));
}
