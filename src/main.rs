mod board;
use board::Board;
use board::Player;

fn main() {
  let board = Board::new(Player::X);
  println!("{:#?}", board);

  let board = board.play(1, Player::X).unwrap();
  println!("{:#?}", board);

  let none = board.play(1, Player::X);
  println!("none {:#?}", none);

  let none = board.play(1, Player::None);
  println!("none {:#?}", none);

  let none = board.play(1, Player::O);
  println!("none {:#?}", none);

  let board = board.play(2, Player::O).unwrap();
  println!("{:#?}", board);
}
