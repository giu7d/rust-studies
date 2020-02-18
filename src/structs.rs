#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct
  fn new(first: &str, last: &str) -> Person {
    return Person {
      first_name: first.to_string(),
      last_name: String::from(last),
    };
  }
  // Get full name
  fn full_name(&self) -> String {
    return format!("{}{}", self.first_name, self.last_name);
  }
  // Set
  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut white = Color {
    red: 255,
    green: 255,
    blue: 255,
  };
  println!("WHITE: {:?}", (white.red, white.green, white.blue));
  // Derive Debug
  println!("DERIVE DEBUG: {:?}", white);
  // Mutate
  white.blue = 0;
  println!("new WHITE: {:?}", (white.red, white.green, white.blue));
  // Constructors
  let mut giu = Person::new("Giu", "7d");
  println!("GIU: {}{}", giu.first_name, giu.last_name);
  // Self
  println!("FULLNAME: {}", giu.full_name());
  // Modify struct value
  giu.set_last_name("Setem"); // To use it you need to set var as multatable
  println!("FULLNAME: {}", giu.full_name());

  println!("TUPLE: {:?}", giu.to_tuple());
}
