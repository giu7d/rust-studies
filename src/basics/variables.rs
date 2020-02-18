pub fn run() {
  // Variables are immutable by default
  let name = "Giu";
  println!("My name is {}", name);

  // Add mutability with mut
  let mut year = 22;
  println!("i was {}", year);
  year = 23;
  println!("Now, i am {}", year);

  // Difine contants
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Giuseppe", 23);
  println!("My name is {}, i am {}", my_name, my_age);
}
