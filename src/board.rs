use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
    None,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Player::X => "X",
            Player::O => "O",
            Player::None => " ",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct Board {
    pub move_count: i32,
    pub next_player: Player,
    pub cells: [Player; 9],
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Won(Player),
    Playing,
}

impl Board {
    pub fn new(next_player: Player) -> Board {
        if next_player == Player::None {
            panic!("None is not a valid starting player")
        }
        Board {
            move_count: 0,
            next_player,
            cells: [
                Player::None,
                Player::None,
                Player::None,
                Player::None,
                Player::None,
                Player::None,
                Player::None,
                Player::None,
                Player::None,
            ],
        }
    }

    pub fn play(&mut self, cell: usize) -> Result<(), String> {
        if self.cells[cell] != Player::None {
            return Err(format!(
                "Cell {} is already in use by {:?}",
                cell, self.cells[cell]
            ));
        }
        let next_player = match self.next_player {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::None => {
                return Err(String::from("Board has an invalid next player"));
            }
        };

        self.move_count += 1;
        self.cells[cell] = self.next_player;
        self.next_player = next_player;
        Ok(())
    }

    pub fn calculate_game_state(&self) -> GameState {
        let winning_states = [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (3, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];

        if self.move_count < 5 {
            return GameState::Playing;
        };

        let cells = self.cells;
        for win_state in winning_states.iter() {
            for player in [Player::X, Player::O].iter() {
                if cells[win_state.0] == *player
                    && cells[win_state.1] == *player
                    && cells[win_state.2] == *player
                {
                    return GameState::Won(*player);
                }
            }
        }

        match self.move_count {
            9 => GameState::Won(Player::None),
            _ => GameState::Playing,
        }
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
        let mut board = Board::new(Player::X);
        board.play(0).unwrap();
        assert_eq!(board.cells[0], Player::X);
        board.play(1).unwrap();
        assert_eq!(board.cells[1], Player::O);
    }

    #[test]
    fn cells_cannot_be_reused() {
        let mut board = Board::new(Player::X);
        board.play(0).unwrap();
        board
            .play(0)
            .expect_err("cell 0 should not have been allowed to be reused");
    }

    #[test]
    fn calculate_game_state_x_win() {
        let mut board = Board::new(Player::X);
        board.play(0).unwrap();
        board.play(1).unwrap();
        board.play(3).unwrap();
        board.play(4).unwrap();
        board.play(6).unwrap();
        assert_eq!(board.calculate_game_state(), GameState::Won(Player::X));
    }
}
