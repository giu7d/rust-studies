// Arrays are dinamic list elements

use std::mem; // You can use a namespace, just like C

pub fn run() {
  // {type;length}
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // To change a value we need to create a let mut ...
  println!("{:?}", numbers);
  //
  // Memory Heap Size
  println!("MEMORY SIZE: {} bytes", mem::size_of_val(&numbers));
  //
  // Slice
  let slice: &[i32] = &numbers[0..3];
  println!("SLICE: {:?}", slice);
  //
  // Push
  numbers.push(6);
  numbers.push(7);
  println!("PUSH: {:?}", numbers);
  //
  // Pop
  numbers.pop();
  println!("POP: {:?}", numbers);
  //
  // Iterations
  println!("LOOP:");
  for n in numbers.iter() {
    println!("it is {}", n);
  }
  // Mutable Iteractions

  for n in numbers.iter_mut() {
    *n *= 2;
  }
  println!("MUT LOOP: {:?}", numbers);
}
