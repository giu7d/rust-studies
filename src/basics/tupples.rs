// Tupples are group of values
// MAX 12 elements
pub fn run() {
  let person: (&str, &str, i8) = ("Giu", "SP", 23);

  println!(
    "{name} is from {uf} and {age} years old",
    name = person.0,
    uf = person.1,
    age = person.2
  );
}
