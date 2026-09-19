use crate::PlayerTurn;
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
    SquareOccupied,
}

#[rustfmt::skip]
pub const KNIGHT_DIRECTION_VECTORS: [(i8,i8);8] = [(2,-1),(2,1),(-2,-1),(-2,1),(-1,2),(-1,-2),(1,2),(1,-2)];
pub const DIAGONAL_DIRECTION_VECTORS: [(i8, i8); 4] = [(1, -1), (1, 1), (-1, -1), (-1, 1)];
pub const HORIZONTAL_DIRECTION_VECTORS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
#[rustfmt::skip]
pub struct Board {
    pub white_pawns:        u64,
    pub white_bishops:      u64,
    pub white_knights:      u64,
    pub white_rooks:        u64,
    pub white_queens:       u64,
    pub white_king:         u64,

    pub black_pawns:        u64,
    pub black_bishops:      u64,
    pub black_knights:      u64,
    pub black_rooks:        u64,
    pub black_queens:       u64,
    pub black_king:         u64,

    pub move_counter:       u64,

    pub white_long_castle_rights:  bool,
    pub white_short_castle_rights: bool,

    pub black_long_castle_rights:  bool,
    pub black_short_castle_rights: bool,
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

            move_counter: 1,

            white_long_castle_rights: true,
            white_short_castle_rights:true,

            black_long_castle_rights: true,
            black_short_castle_rights:true

        }
    }
    pub fn pawn_valid_move(
        &self,
        file: &char,
        rank: &char,
        turn: &PlayerTurn,
    ) -> Result<Square, BoardError> {
        let rank_as_digit = *rank as u8 - b'0';
        let current_square =
            Self::translate_rank_to_bits(rank) + Self::translate_file_to_bits(file);
        if !(self.check_square_available(&current_square)) {
            return Err(BoardError::SquareOccupied);
        }
        match *turn {
            PlayerTurn::Black => {
                if rank_as_digit >= 7 {
                    return Err(BoardError::InputError);
                }
                if self.black_pawns & 1u64 << (current_square + 8) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit + 1,
                    })
                } else if self.black_pawns & 1u64 << (current_square + 16) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit + 2,
                    })
                } else {
                    Err(BoardError::NoPiece)
                }
            }
            PlayerTurn::White => {
                if rank_as_digit <= 1 || rank_as_digit > 8 {
                    return Err(BoardError::InputError);
                }

                if self.white_pawns & (1u64 << (current_square - 8)) != 0 {
                    Ok(Square {
                        file: *file,
                        rank: rank_as_digit - 1,
                    })
                } else if self.white_pawns & 1u64 << (current_square - 16) != 0 {
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
    /*
     * Directions are flipped, for example (1,1) normally means going (right -> down) but since it
     * is a bitboard, it is flipped to go (right -> up).
     *  return value will be the opposite direction as we are currently checking from (file,rank) to piece.
     * */
    pub fn piece_moves_check(
        piece: &u64,
        file: &char,
        rank: &char,
        directions: &[(i8, i8); 4],
    ) -> Option<[(i8, i8); 1]> {
        let current_rank = Self::translate_rank_to_numeric(rank);
        let current_file = Self::translate_file_to_bits(file) as i8;
        eprintln!("{} {}", current_rank, current_file);
        for (dx, dy) in directions {
            for scalar in 1i8..8 {
                let new_file = current_file + (dx * scalar);
                let new_rank = current_rank + (dy * scalar);

                if !(0..8).contains(&new_file) || !(0..8).contains(&new_rank) {
                    break;
                }
                let new_square = (new_rank * 8 + new_file) as u64;
                if piece & (1u64 << new_square) != 0 {
                    return Some([(-dx, -dy)]);
                }
            }
        }
        None
    }
    pub fn knight_moves_check(
        piece: &u64,
        file: &char,
        rank: &char,
        directions: &[(i8, i8); 8],
    ) -> bool {
        let current_rank = Self::translate_rank_to_numeric(rank);
        let current_file = Self::translate_file_to_bits(file) as i8;
        for (dx, dy) in directions {
            let new_file = current_file + dx;
            let new_rank = current_rank + dy;

            if !(0..8).contains(&new_file) || !(0..8).contains(&new_rank) {
                continue;
            }
            let new_square = (new_rank * 8 + new_file) as u64;
            if piece & (1u64 << new_square) != 0 {
                return true;
            }
        }
        false
    }

    pub fn move_obstruction_check(
        &self,
        file: &char,
        rank: &char,
        directions: &[(i8, i8); 1],
    ) -> bool {
        let current_rank = Self::translate_rank_to_numeric(rank);
        let current_file = Self::translate_file_to_bits(file) as i8;
        for (dx, dy) in directions {
            for scalar in 1i8..8 {
                let new_file = current_file + (dx * scalar);
                let new_rank = current_rank + (dy * scalar);

                if !(0..8).contains(&new_file) || !(0..8).contains(&new_rank) {
                    break;
                }
                let new_square = (new_rank * 8 + new_file) as u64;

                if !(Self::check_square_available(self, &new_square)) {
                    return false;
                }
            }
        }
        true
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
    pub fn translate_rank_to_numeric(rank: &char) -> i8 {
        match *rank {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => 0,
        }
    }
    /*
     * This function is to check for any square, are there any pieces currently
     * on it.
     * Returns a boolean and allows you to use it from there for capturing or moving of pieces
     * */
    pub fn check_square_available(&self, square: &u64) -> bool {
        self.white_pawns & (1u64 << square) == 0
            && self.white_king & (1u64 << square) == 0
            && self.white_rooks & (1u64 << square) == 0
            && self.white_queens & (1u64 << square) == 0
            && self.white_knights & (1u64 << square) == 0
            && self.white_bishops & (1u64 << square) == 0
            && self.black_pawns & (1u64 << square) == 0
            && self.black_king & (1u64 << square) == 0
            && self.black_rooks & (1u64 << square) == 0
            && self.black_queens & (1u64 << square) == 0
            && self.black_knights & (1u64 << square) == 0
            && self.black_bishops & (1u64 << square) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board;

    #[test]
    fn knight_check() {
        let board = Board::new();
        let check = board::Board::knight_moves_check(
            &board.white_knights,
            &'c',
            &'3',
            &KNIGHT_DIRECTION_VECTORS,
        );
        assert!(check);
    }
    #[test]
    fn bishop_check() {
        let board = Board::new();
        let check = board::Board::piece_moves_check(
            &board.white_bishops,
            &'h',
            &'3',
            &DIAGONAL_DIRECTION_VECTORS,
        );
        println!("{:?}", check);
        assert!(check.is_some());
    }
    #[test]
    fn rook_check() {
        let board = Board::new();
        let check = board::Board::piece_moves_check(
            &board.white_rooks,
            &'e',
            &'5',
            &HORIZONTAL_DIRECTION_VECTORS,
        );
        println!("{:?}", check);
        assert!(check.is_some());
    }
}
