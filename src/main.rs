#![allow(dead_code)]
use crate::board::Board;
use std::io::Write;

mod board;
mod input_move;

struct Move {
    from: Square,
    to: Square,
    promotion: Option<Piece>,
}
enum Piece {
    Bishop,
    Knight,
    Queen,
    Rook,
}
struct Square {
    file: char,
    rank: u8,
}
fn main() {
    let mut chess_board = Board::new();

    loop {
        let mut input_move = String::new();

        print!("Enter your move: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input_move)
            .expect("Failed to read lien");
        let input_move_trimmed = input_move.trim();
        if check_input_valid(input_move_trimmed)
            && input_move::valid_input_moves(input_move_trimmed)
        {
        } else {
            eprintln!("{} is not a valid move", input_move_trimmed);
        }
    }
}

fn check_input_valid(input: &str) -> bool {
    if input.len() == 1 || input.len() >= 7 {
        return false;
    }
    true
}
