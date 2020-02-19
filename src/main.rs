extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod board;
use board::{Board, GameState, Player};

use std::io::{stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut board = Board::new(Player::X);

    let mut cursor = 0;
    print_board(&mut stdout, &board, cursor);
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') | Key::Esc => break,
            Key::Left => {
                if cursor != 0 {
                    cursor -= 1
                }
            }
            Key::Right => {
                if cursor < 8 {
                    cursor += 1
                }
            }
            Key::Up => {
                if cursor >= 3 {
                    cursor -= 3
                }
            }
            Key::Down => {
                if cursor < 6 {
                    cursor += 3
                }
            }
            Key::Char(' ') | Key::Char('\n') => {
                match board.play(cursor) {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
            _ => {}
        };
        print_board(&mut stdout, &board, cursor);
        stdout.flush().unwrap();
        match board.calculate_game_state() {
            GameState::Won(winner) => {
                print_won(&board, winner);
                break;
            }
            GameState::Playing => {}
        }
    }
}

fn print_won(board: &Board, winner: Player) {
    print!("\n");
    match winner {
        Player::None => println!("Tie!"),
        _ => println!("{} won in {} moves!", winner, board.move_count()),
    }
}

fn print_board(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    board: &Board,
    cursor: usize,
) {
    write!(
        stdout,
        "{}{}q to exit, arrow keys to move, enter to play\n\n\r",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )
    .unwrap();
    stdout.flush().unwrap();

    print!(
        "{} | {} | {}\n\r---------\n\r{} | {} | {}\n\r---------\n\r{} | {} | {}\n\r",
        display_cell(board, cursor, 0),
        display_cell(board, cursor, 1),
        display_cell(board, cursor, 2),
        display_cell(board, cursor, 3),
        display_cell(board, cursor, 4),
        display_cell(board, cursor, 5),
        display_cell(board, cursor, 6),
        display_cell(board, cursor, 7),
        display_cell(board, cursor, 8),
    );
}

fn display_cell(board: &Board, cursor: usize, cell: usize) -> String {
    let cells = board.cells();
    if cursor == cell {
        let cell_display = if cells[cell] == Player::None {
            board.next_player()
        } else {
            cells[cell]
        };
        return format!(
            "{}{}{}",
            termion::style::Underline,
            cell_display,
            termion::style::Reset
        );
    }
    format!("{}", cells[cell])
}
