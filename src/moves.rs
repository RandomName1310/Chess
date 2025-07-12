// move logic
pub type Moveset = &'static[Offset];

// offset moves 
pub struct Offset {
    pub dx: i8,
    pub dy: i8,
}

// pawn moves
pub const PAWN_WHITE_MOVES: Moveset = &[Offset { dx: 0, dy: -1 }];
pub const PAWN_BLACK_MOVES: Moveset = &[Offset { dx: 0, dy: 1 }];
pub const PAWN_CAPTURES: Moveset = &[Offset { dx: 1, dy: 1 }, Offset { dx: -1, dy: 1 }];

// bishop moves
pub const BISHOP_MOVES: Moveset= &[
    Offset { dx: 1, dy: 1 },
    Offset { dx: 1, dy: -1 },
    Offset { dx: -1, dy: 1 },
    Offset { dx: -1, dy: -1 },
];

pub const ROOK_MOVES: Moveset= &[
    Offset { dx: 1, dy: 0 },
    Offset { dx: -1, dy: 0 },
    Offset { dx: 0, dy: 1 },
    Offset { dx: 0, dy: -1 },
];

// Knight moves
pub const KNIGHT_MOVES: Moveset = &[
    Offset { dx: 1, dy: 2 },
    Offset { dx: 2, dy: 1 },
    Offset { dx: -1, dy: 2 },
    Offset { dx: -2, dy: 1 },
    Offset { dx: 1, dy: -2 },
    Offset { dx: 2, dy: -1 },
    Offset { dx: -1, dy: -2 },
    Offset { dx: -2, dy: -1 },
];

// King moves
pub const KING_MOVES: &[Offset] = &[
    Offset { dx: 1, dy: 0 },
    Offset { dx: 1, dy: 1 },
    Offset { dx: 0, dy: 1 },
    Offset { dx: -1, dy: 1 },
    Offset { dx: -1, dy: 0 },
    Offset { dx: -1, dy: -1 },
    Offset { dx: 0, dy: -1 },
    Offset { dx: 1, dy: -1 },
];

// queen move
pub const QUEEN_MOVES: Moveset = &[
    // straight
    Offset { dx: 1, dy: 0 },
    Offset { dx: -1, dy: 0 },
    Offset { dx: 0, dy: 1 },
    Offset { dx: 0, dy: -1 },
    // diagonal
    Offset { dx: 1, dy: 1 },
    Offset { dx: 1, dy: -1 },
    Offset { dx: -1, dy: 1 },
    Offset { dx: -1, dy: -1 },
];

// pawn special moves    
pub const PAWN_DOUBLE_FORWARD: Offset = Offset { dx: 0, dy: 2 };
pub const PAWN_CAPTURE_LEFT: Offset = Offset { dx: -1, dy: 1 };
pub const PAWN_CAPTURE_RIGHT: Offset = Offset { dx: 1, dy: 1 };