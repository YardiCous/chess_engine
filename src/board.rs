pub enum PlayerTurn {
    White,
    Black,
}
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
#[derive(Debug)]
pub struct Square {
    file: char,
    rank: u8,
}
#[derive(Debug)]
pub enum BoardError {
    NoPiece,
    InputError,
}
pub struct Board {
    pub white_pawns: u64,
    pub white_bishops: u64,
    pub white_knights: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_bishops: u64,
    pub black_knights: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,
}
/* A B C D E F G H
* [R,K,B,Q K B K R] 8
* [P,P,P,P,P,P,P,P] 7
* [0,0,0,0,0,0,0,0] 6
* [0,0,0,0,0,0,0,0] 5
* [0,0,0,0,0,0,0,0] 4
* [0,0,0,0,0,0,0,0] 3
* [P,P,P,P,P,P,P,P] 2
* [R,K,B,Q K B K R] 1
*/

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            white_pawns:   0x000000000000FF00,
            white_bishops: 0x0000000000000024,
            white_knights: 0x0000000000000042,
            white_rooks:   0x0000000000000081,
            white_queens:  0x0000000000000008,
            white_king:    0x0000000000000010,

            black_pawns:   0x00FF000000000000,
            black_bishops: 0x2400000000000000,
            black_knights: 0x4200000000000000,
            black_rooks:   0x8100000000000000,
            black_queens:  0x0800000000000000,
            black_king:    0x1000000000000000,
        }
    }
    pub fn pawn_move_valid_move(
        &self,
        file: &char,
        rank: &char,
        turn: &PlayerTurn,
    ) -> Result<Square, BoardError> {
        let rank_as_digit = *rank as u8 - b'0';
        match *turn {
            PlayerTurn::Black => {
                let total_bits =
                    Self::translate_rank_to_bits(rank) + Self::translate_file_to_bits(file);

                if rank_as_digit >= 7 {
                    return Err(BoardError::InputError);
                }
                if self.black_pawns & 1u64 << (total_bits + 8) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit + 1,
                    })
                } else if self.black_pawns & 1u64 << (total_bits + 16) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit + 2,
                    })
                } else {
                    Err(BoardError::NoPiece)
                }
            }
            PlayerTurn::White => {
                let total_bits =
                    Self::translate_rank_to_bits(rank) + Self::translate_file_to_bits(file);
                if rank_as_digit <= 1 || rank_as_digit > 8 {
                    return Err(BoardError::InputError);
                }

                if self.white_pawns & (1u64 << (total_bits - 8)) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit - 1,
                    })
                } else if self.white_pawns & 1u64 << (total_bits - 16) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit - 2,
                    })
                } else {
                    Err(BoardError::NoPiece)
                }
            }
        }
    }
    pub fn translate_rank_to_bits(rank: &char) -> u64 {
        match *rank {
            '1' => 0,
            '2' => 8,
            '3' => 16,
            '4' => 24,
            '5' => 32,
            '6' => 40,
            '7' => 48,
            '8' => 56,

            _ => 0,
        }
    }
    pub fn translate_file_to_bits(file: &char) -> u64 {
        match *file {
            'a' => 7,
            'b' => 6,
            'c' => 5,
            'd' => 4,
            'e' => 3,
            'f' => 2,
            'g' => 1,
            'h' => 0,
            _ => 0,
        }
    }
}
