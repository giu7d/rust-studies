use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let first_command = args[1].clone();

  println!("ARGS: {:#?}", args);
  println!("COMMAND: {:?}", first_command);

  if first_command == "hello" {
    println!("Say Hello World");
  }
}
