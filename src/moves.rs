// move logic
// offset moves 
pub struct Offset {
    dx: i8,
    dy: i8,
}

// pawn moves
pub const PAWN_MOVES: &[Offset] = &[Offset { dx: 0, dy: 1 }];
pub const PAWN_CAPTURES: &[Offset] = &[Offset { dx: 1, dy: 1 }, Offset { dx: -1, dy: 1 }];

// bishop moves
v const BISHOP_MOVES: &[Offset] = &[
    Offset { dx: 1, dy: 1 },
    Offset { dx: 1, dy: -1 },
    Offset { dx: -1, dy: 1 },
    Offset { dx: -1, dy: -1 },
];

// Knight moves
pub const KNIGHT_MOVES: &[Offset] = &[
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
pub const QUEEN_MOVES: &[Offset] = &[
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