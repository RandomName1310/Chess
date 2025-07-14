// move logic
pub type Moveset = &'static[Offset];

// offset moves 
#[derive(PartialEq)]
pub struct Offset {
    pub dx: i8,
    pub dy: i8,
    pub repeats: i8,
}

// pawn moves
pub const PAWN_WHITE_MOVES: Moveset = &[Offset { dx: 0, dy: -1, repeats: 1}];
pub const PAWN_BLACK_MOVES: Moveset = &[Offset { dx: 0, dy: 1, repeats: 1}];
pub const PAWN_WHITE_CAPTURES: Moveset = &[Offset { dx: 1, dy: -1, repeats: 1}, Offset { dx: -1, dy: -1, repeats: 1}];
pub const PAWN_BLACK_CAPTURES: Moveset = &[Offset { dx: 1, dy: 1, repeats: 1}, Offset { dx: -1, dy: 1, repeats: 1}];

// bishop moves
pub const BISHOP_MOVES: Moveset= &[
    Offset { dx: 1, dy: 1, repeats: 10},
    Offset { dx: 1, dy: -1, repeats: 10},
    Offset { dx: -1, dy: 1, repeats: 10},
    Offset { dx: -1, dy: -1, repeats: 10},
];

pub const ROOK_MOVES: Moveset= &[
    Offset { dx: 1, dy: 0, repeats: 10},
    Offset { dx: -1, dy: 0, repeats: 10},
    Offset { dx: 0, dy: 1, repeats: 10},
    Offset { dx: 0, dy: -1, repeats: 10},
];

// Knight moves
pub const KNIGHT_MOVES: Moveset = &[
    Offset { dx: 1, dy: 2, repeats: 1},
    Offset { dx: 2, dy: 1, repeats: 1},
    Offset { dx: -1, dy: 2, repeats: 1},
    Offset { dx: -2, dy: 1, repeats: 1},
    Offset { dx: 1, dy: -2, repeats: 1},
    Offset { dx: 2, dy: -1, repeats: 1},
    Offset { dx: -1, dy: -2, repeats: 1},
    Offset { dx: -2, dy: -1, repeats: 1},
];

// King moves
pub const KING_MOVES: &[Offset] = &[
    Offset { dx: 1, dy: 0, repeats: 1},
    Offset { dx: 1, dy: 1, repeats: 1},
    Offset { dx: 0, dy: 1, repeats: 1},
    Offset { dx: -1, dy: 1, repeats: 1},
    Offset { dx: -1, dy: 0, repeats: 1},
    Offset { dx: -1, dy: -1, repeats: 1},
    Offset { dx: 0, dy: -1, repeats: 1},
    Offset { dx: 1, dy: -1, repeats: 1},
];

// queen move
pub const QUEEN_MOVES: Moveset = &[
    // straight
    Offset { dx: 1, dy: 0, repeats: 10},
    Offset { dx: -1, dy: 0, repeats: 10},
    Offset { dx: 0, dy: 1, repeats: 10},
    Offset { dx: 0, dy: -1, repeats: 10},
    // diagonal
    Offset { dx: 1, dy: 1, repeats: 10},
    Offset { dx: 1, dy: -1, repeats: 10},
    Offset { dx: -1, dy: 1, repeats: 10},
    Offset { dx: -1, dy: -1, repeats: 10},
];

// pawn special moves    
pub const PAWN_WHITE_DOUBLE_FORWARD: Moveset = &[Offset { dx: 0, dy: -1, repeats: 2}];
pub const PAWN_BLACK_DOUBLE_FORWARD: Moveset = &[Offset { dx: 0, dy: 1, repeats: 2}];