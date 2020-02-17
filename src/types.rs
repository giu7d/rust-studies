/*
*   Primative Types
*
*   - Ints: u8, i8, i16, u32, i32, u64, i64, u128, i128 (number of bits in memory)
*   - Floats: f32, f64
*   - Boolean: bool
*   - Characters: char
*   - Tuples
*   - Arrays (Fix Length)
*/

// Rust is statically typed!! But the compiler can infer types dynamically

pub fn run() {
  // Find Max Size
  println!("Max i16: {}", std::i16::MAX);
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);
  println!("Max i128: {}", std::i128::MAX);

  // Boolean
  let is_activate: bool = true;
  // Boolean from expression
  let is_greater: bool = 10 < 5;
  // Chars & Unicode
  let a = 'a';
  let face = '\u{1F600}';

  println!("{:#?}", (is_activate, is_greater, a, face))
}
