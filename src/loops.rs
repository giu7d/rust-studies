pub fn run() {
  let mut count = 0;

  // Infinete Loop
  println!("INFINETE LOOP:");
  loop {
    count += 1;
    println!("it's {}", count);
    if count == 5 {
      break;
    }
  }

  // While Loop (FizzBuzz)
  count = 0;
  println!("\nWHILE LOOP:");
  while count <= 10 {
    if count % 3 == 0 && count % 5 == 0 {
      println!("FizzBuzz")
    } else if count % 3 == 0 {
      println!("Fizz")
    } else if count % 5 == 0 {
      println!("Buzz")
    } else {
      println!("{}", count)
    }
    count += 1;
  }

  // For Range
  println!("\nFOR LOOP:");
  for num in 0..10 {
    if num % 3 == 0 && num % 5 == 0 {
      println!("FizzBuzz")
    } else if num % 3 == 0 {
      println!("Fizz")
    } else if num % 5 == 0 {
      println!("Buzz")
    } else {
      println!("{}", num)
    }
  }
}
