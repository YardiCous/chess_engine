pub enum PlayerTurn {
    White,
    Black,
}

impl PlayerTurn {
    pub fn change_player_turn(&mut self) {
        match self {
            PlayerTurn::White => *self = PlayerTurn::Black,
            PlayerTurn::Black => *self = PlayerTurn::White,
        }
    }
}
