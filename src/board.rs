#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
  X,
  O,
  None,
}

#[derive(Debug)]
pub struct Board {
  move_count: i32,
  next_player: Player,
  cells: [Player; 9],
}

impl Board {

  pub fn new(next_player: Player) -> Board {
    if next_player == Player::None {
      panic!("None is not a valid starting player")
    }
    Board {
      move_count: 0,
      next_player,
      cells: [Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None, Player::None]
    }
  }

  pub fn play(&self, cell: usize, player: Player) -> Result<Board, String> {
    if self.next_player != player {
      return Err(format!("Next Player should be {:?} got {:?}", self.next_player, player));
    }
    if self.cells[cell] != Player::None {
      return Err(format!("Cell {} is already in use by {:?}", cell, player));
    }
    let next_player = match player {
      Player::X => Player::O,
      Player::O => Player::X,
      Player::None => {
        return Err("Board has an invalid next player".to_string());
      }
    };

    let mut board = Board {
      move_count: self.move_count + 1,
      next_player,
      ..*self
    };
    board.cells[cell] = player;
    Ok(board)
  }

  pub fn won(&self) -> Player {
    Player::X
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_works() {
    Board::new(Player::X);
  }

  #[test]
  #[should_panic]
  fn new_none() {
    Board::new(Player::None);
  }

  #[test]
  fn play_works() {
    let board = Board::new(Player::X);
    let board = board.play(0, Player::X).unwrap();
    assert_eq!(board.cells[0], Player::X);
    let board = board.play(1, Player::O).unwrap();
    assert_eq!(board.cells[1], Player::O);
  }

  #[test]
  fn players_must_take_turns() {
    let board = Board::new(Player::X);
    board.play(0, Player::O).expect_err("player O should not have been allowed to play");
    let board = board.play(0, Player::X).unwrap();
    board.play(2, Player::X).expect_err("player X should not have been allowed to play");
    board.play(1, Player::O).unwrap();
  }

  #[test]
  fn cells_cannot_be_reused() {
    let board = Board::new(Player::X);
    let board = board.play(0, Player::X).unwrap();
    board.play(0, Player::O).expect_err("cell 0 should not have been allowed to be reused");
  }
}
