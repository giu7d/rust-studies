mod arrays;
mod cli_args;
mod conditions;
mod enums;
mod functions;
mod loops;
mod pointers;
mod print;
mod strings;
mod structs;
mod tupples;
mod types;
mod variables;
mod vectors;

pub fn run() {
  print::run();
  variables::run();
  types::run();
  strings::run();
  tupples::run();
  arrays::run();
  vectors::run();
  conditions::run();
  loops::run();
  functions::run();
  pointers::run();
  structs::run();
  enums::run();
  cli_args::run();
}
