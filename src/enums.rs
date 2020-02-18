enum Moves {
  // variants
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: &Moves) {
  // Perform action depend on info
  match m {
    Moves::Up => println!("Moving Up..."),
    Moves::Down => println!("Moving Down..."),
    Moves::Left => println!("Moving Left..."),
    Moves::Right => println!("Moving Right..."),
  }
}

pub fn run() {
  let avatar = [
    Moves::Up,
    Moves::Down,
    Moves::Left,
    Moves::Left,
    Moves::Right,
  ];
  let avatar2 = Moves::Down;

  for moves in avatar.iter() {
    move_avatar(moves);
  }

  move_avatar(&avatar2);
}
