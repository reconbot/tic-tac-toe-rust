fn main() {
  let board = Board::build(Player::X);
  println!("{:#?}", board);

  let board = board.play(1, Player::X).unwrap();
  println!("{:#?}", board);

  let none = board.play(1, Player::X);
  println!("none {:#?}", none);

  let none = board.play(1, Player::None);
  println!("none {:#?}", none);

  let none = board.play(1, Player::Y);
  println!("none {:#?}", none);

  let board = board.play(2, Player::Y).unwrap();
  println!("{:#?}", board);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
  X,
  Y,
  None,
}

#[derive(Debug)]
struct Board {
  move_count: i32,
  next_player: Player,
  cells: [Player; 9],
}

impl Board {

  fn build(next_player: Player) -> Board {
    Board {
      move_count: 0,
      next_player,
      cells: [Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None]
    }
  }

  fn play(&self, cell: usize, player: Player) -> Option<Board> {
    if self.next_player != player {
      return None;
    }
    if self.cells[cell] != Player::None {
      return None
    }
    let next_player = match player {
      Player::X => Player::Y,
      Player::Y => Player::X,
      Player::None => {
        return None;
      }
    };

    let mut board = Board {
      move_count: self.move_count + 1,
      next_player,
      ..*self
    };
    board.cells[cell] = player;
    Some(board)
  }

  fn won(&self) -> Player {
    Player::X
  }
}
