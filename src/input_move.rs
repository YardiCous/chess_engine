macro_rules! valid_chars {
    ($name:ident,$chars:literal) => {
        fn $name(c: &char) -> bool {
            $chars.contains(*c)
        }
    };
}
valid_chars!(valid_files, "abcdefgh");
valid_chars!(valid_rank, "12345678");
valid_chars!(valid_piece, "NKQBR");
pub fn valid_input_moves(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    match chars.as_slice() {
        /*
         * For pawn moves
         * */
        [file, rank] if valid_files(file) && valid_rank(rank) => true,

        /*
         * For Castling
         * */
        ['O', '-', 'O'] => true,
        /*
         * For piece move
         * */
        [piece, file, rank] if valid_piece(piece) && valid_files(file) && valid_rank(rank) => true,
        /*
         * For promotion of pawn
         * */
        [pawn_file, rank, equal, piece]
            if valid_files(pawn_file)
                && valid_rank(rank)
                && *equal == '='
                && valid_piece(piece) =>
        {
            true
        }
        /*
         * For two of more pieces that can move the same square
         * */
        [piece, piece_file, file, rank]
            if valid_piece(piece)
                && valid_files(piece_file)
                && valid_files(file)
                && valid_rank(rank) =>
        {
            true
        }
        /*
         * For piece capture
         * */
        [piece, takes, file, rank]
            if valid_piece(piece) && *takes == 'x' && valid_files(file) && valid_rank(rank) =>
        {
            true
        }

        /*
         * For pawn capture
         * */
        [pawn_file, takes, file, rank]
            if valid_files(pawn_file) && *takes == 'x' && valid_files(file) && valid_rank(rank) =>
        {
            true
        }
        /*
         * For long castle
         * */
        ['O', '-', 'O', '-', 'O'] => true,
        /*
         * For multiple pieces take can move to the same square
         * */
        [piece, piece_file, piece_rank, file, rank]
            if valid_piece(piece)
                && valid_files(piece_file)
                && valid_rank(piece_rank)
                && valid_files(file)
                && valid_rank(rank) =>
        {
            true
        }
        /*
         * For pawn capture into promotion
         * */
        [pawn_file, takes, file, rank, equal, piece]
            if valid_files(pawn_file)
                && *takes == 'x'
                && valid_files(file)
                && valid_rank(rank)
                && *equal == '='
                && valid_piece(piece) =>
        {
            true
        }
        /*
         * For multiple pieces that can capture the same square
         * */
        [piece, piece_file, piece_rank, takes, file, rank]
            if valid_piece(piece)
                && valid_files(piece_file)
                && valid_rank(piece_rank)
                && *takes == 'x'
                && valid_files(file)
                && valid_rank(rank) =>
        {
            true
        }
        _ => false,
    }
}
