// Arrays are fixed list elements
use std::mem; // You can use a namespace, just like C

pub fn run() {
  // {type;length}
  let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // To change a value we need to create a let mut ...
  println!("{:?}", numbers);

  for n in &numbers {
    println!("it is {}", n);
  }

  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Slice
  let slice: &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice);
}
