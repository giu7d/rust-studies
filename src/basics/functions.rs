pub fn run() {
  hello("nice to meet u!", "Giu");
  println!("ADD(): {}", add(1, 2));

  // Closure
  let add_closure = |n1: i32, n2: i32| n1 + n2;
  println!("ADD_CLOSURE(): {}", add_closure(2, 2));
}

fn hello(msg: &str, name: &str) {
  println!("HELLO(): {name}, {msg}", name = name, msg = msg);
}

fn add(n1: i32, n2: i32) -> i32 {
  let n3 = n1 + 1;
  return n3 + n2;
}
