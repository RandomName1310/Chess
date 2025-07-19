use serde::{Serialize, Deserialize};
use crate::moves::*;

pub const BOARD_SIZE: usize = 8;
pub const SQUARE_SIZE: f32 = 80.0;

#[derive(Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn(Moveset),
    Rook(Moveset),
    Knight(Moveset),
    Bishop(Moveset),
    Queen(Moveset),
    King(Moveset),
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub x: usize, 
    pub y: usize,
    pub piece_type: PieceType,
    pub color: PieceColor, 
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Square {
    pub color: char, 
    pub piece: char, 
    pub state: char, 
}

pub type Board = [[Square; 8]; 8];

pub const BOARD_LAYOUT: Board = [
    // 0
    [
        Square { color: 'B', piece: 'R', state: '_' },
        Square { color: 'W', piece: 'N', state: '_' },
        Square { color: 'B', piece: 'B', state: '_' },
        Square { color: 'W', piece: 'Q', state: '_' },
        Square { color: 'B', piece: 'K', state: '_' },
        Square { color: 'W', piece: 'B', state: '_' },
        Square { color: 'B', piece: 'N', state: '_' },
        Square { color: 'W', piece: 'R', state: '_' },
    ],
    // 1
    [
        Square { color: 'W', piece: 'P', state: '_' },
        Square { color: 'B', piece: 'P', state: '_' },
        Square { color: 'W', piece: 'P', state: '_' },
        Square { color: 'B', piece: 'P', state: '_' },
        Square { color: 'W', piece: 'P', state: '_' },
        Square { color: 'B', piece: 'P', state: '_' },
        Square { color: 'W', piece: 'P', state: '_' },
        Square { color: 'B', piece: 'P', state: '_' },
    ],
    //2
    [
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
    ],
    // 3
    [
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
    ],
    // 4
    [
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
    ],
    // 5
    [
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
        Square { color: 'W', piece: ' ', state: '_' },
        Square { color: 'B', piece: ' ', state: '_' },
    ],
    // 6
    [
        Square { color: 'B', piece: 'p', state: '_' },
        Square { color: 'W', piece: 'p', state: '_' },
        Square { color: 'B', piece: 'p', state: '_' },
        Square { color: 'W', piece: 'p', state: '_' },
        Square { color: 'B', piece: 'p', state: '_' },
        Square { color: 'W', piece: 'p', state: '_' },
        Square { color: 'B', piece: 'p', state: '_' },
        Square { color: 'W', piece: 'p', state: '_' },
    ],
    // 7
    [
        Square { color: 'W', piece: 'r', state: '_' },
        Square { color: 'B', piece: 'n', state: '_' },
        Square { color: 'W', piece: 'b', state: '_' },
        Square { color: 'B', piece: 'q', state: '_' },
        Square { color: 'W', piece: 'k', state: '_' },
        Square { color: 'B', piece: 'b', state: '_' },
        Square { color: 'W', piece: 'n', state: '_' },
        Square { color: 'B', piece: 'r', state: '_' },
    ],
];

/* private functions */

fn get_piece_type(piece_char: char, color: PieceColor, y: usize) -> PieceType{
    let piece_type = match piece_char.to_ascii_lowercase() {
        'p' => {
            if color == PieceColor::Black {
                if y == 1 { PieceType::Pawn(PAWN_BLACK_DOUBLE_FORWARD) }
                else      { PieceType::Pawn(PAWN_BLACK_MOVES) }
            } else {
                if y == 6 { PieceType::Pawn(PAWN_WHITE_DOUBLE_FORWARD) }
                else      { PieceType::Pawn(PAWN_WHITE_MOVES) }
            }
        }
        'r' => PieceType::Rook(ROOK_MOVES),
        'n' => PieceType::Knight(KNIGHT_MOVES),
        'b' => PieceType::Bishop(BISHOP_MOVES),
        'q' => PieceType::Queen(QUEEN_MOVES),
        'k' => PieceType::King(KING_MOVES),
        _ => PieceType::Empty,
    };

    piece_type
}

// calculate state of all Moveset offsets
fn calculate_offsets(board: &mut Board, 
                     path: Moveset, 
                     color: PieceColor, 
                     white_attacks: &mut [[bool; 8]; 8], 
                     black_attacks: &mut [[bool; 8]; 8], 
                     (x, y): (usize, usize), 
                     color_this_path: bool){
    // Go through each move offset
    for offset in path {
        let mut pos_x = x as isize;
        let mut pos_y = y as isize;

        for _ in 0..offset.repeats {
            pos_x += offset.dx as isize;
            pos_y += offset.dy as isize;

            if pos_x < 0 || pos_x >= BOARD_SIZE as isize || pos_y < 0 || pos_y >= BOARD_SIZE as isize {
                break;
            }

            let tx = pos_x as usize;
            let ty = pos_y as usize;


            let target_piece = board[ty][tx].piece;

            let is_enemy = match color {
                PieceColor::White => target_piece.is_uppercase(),
                PieceColor::Black => target_piece.is_lowercase(),
            };

            if color_this_path && (target_piece == ' ' || is_enemy) {
                color_square(board, (tx, ty), 'R');
            }

            if target_piece != ' ' {
                if (color == PieceColor::White && target_piece.is_uppercase()) ||
                    (color == PieceColor::Black && target_piece.is_lowercase()) {
                    match color {
                        PieceColor::White => white_attacks[ty][tx] = true,
                        PieceColor::Black => black_attacks[ty][tx] = true,
                    }
                }
                break; // can't jump over pieces
            }

            // empty tile is always reachable
            match color {
                PieceColor::White => white_attacks[ty][tx] = true,
                PieceColor::Black => black_attacks[ty][tx] = true,
            }
        }
    }
}


// calculate state of special pawn capture
fn calculate_pawn_capture(piece: Piece, board: &mut Board,
                     color: PieceColor, 
                     white_attacks: &mut [[bool; 8]; 8], 
                     black_attacks: &mut [[bool; 8]; 8], 
                     (x, y): (usize, usize)){
    let captures = match color {
        PieceColor::White => PAWN_WHITE_CAPTURES,
        PieceColor::Black => PAWN_BLACK_CAPTURES,
    };

    for offset in captures {
        let tx = x as isize + offset.dx as isize;
        let ty = y as isize + offset.dy as isize;

        // outside the board?
        if tx < 0 || tx >= BOARD_SIZE as isize || ty < 0 || ty >= BOARD_SIZE as isize {
            continue;
        }

        let (tx, ty) = (tx as usize, ty as usize);
        let target_piece = board[ty][tx].piece;

        // skip empty squares
        if target_piece == ' ' {
            continue;
        }

        // mark only if the piece is an opponent’s
        let is_enemy = match color {
            PieceColor::White => target_piece.is_uppercase(), 
            PieceColor::Black => target_piece.is_lowercase(),
        };

        if is_enemy && piece.x == x && piece.y == y{
            color_square(board, (tx as usize, ty as usize), 'R');
            match color {
                PieceColor::White => white_attacks[ty][tx] = true,
                PieceColor::Black => black_attacks[ty][tx] = true,
            }
        }
    }
}

/* public functions */

// color specific square
pub fn color_square(board: &mut Board, coord: (usize, usize), color: char){
    board[coord.1][coord.0].color = color;
}

// sets the "state" parameter on main board and color square of pieces-> which piece attack where
pub fn set_board_state(board: &mut Board, piece: Piece){
    for row in board.iter_mut() {
        for square in row.iter_mut() {
            square.state = '_';
        }
    }

    // Temporary buffer to track which side attacks which square
    let mut white_attacks = [[false; 8]; 8];
    let mut black_attacks = [[false; 8]; 8];

    color_square(board, (piece.x, piece.y), 'G');

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            // get char in the specific square
            let piece_char = board[y][x].piece;

            if piece_char == ' ' {
                continue; 
            }

            // get color
            let color = if piece_char.is_uppercase() {
                PieceColor::Black
            } else {
                PieceColor::White
            };

            // get the PieceType to unwrap after
            let piece_type = get_piece_type(piece_char, color, y);

            // unwraps PieceType
            let path = match piece_type {
                PieceType::Pawn(m) |
                PieceType::Rook(m) |
                PieceType::Knight(m) |
                PieceType::Bishop(m) |
                PieceType::Queen(m) |
                PieceType::King(m) => m,
                PieceType::Empty => continue,
            };

            // check if the path of a piece can be colored
            let mut color_this_path = false;

            if (x, y) == (piece.x, piece.y){
                color_this_path = true;
            }

            // calculates the offsets of the pieces and colors the squares
            calculate_offsets(board,
                path,
                color,
                &mut white_attacks,
                &mut black_attacks,
                (x, y),
                color_this_path,);

            // calculate special pawn diagonal captures
            if let PieceType::Pawn(_) = piece_type {
                calculate_pawn_capture(piece,
                board,
                color,
                &mut white_attacks,
                &mut black_attacks,
                (x, y));
            }
        }
    }

    // apply the final state per square
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            board[y][x].state = match (white_attacks[y][x], black_attacks[y][x]) {
                (true, true)   => 'T',
                (true, false)  => 'W',
                (false, true)  => 'B',
                (false, false) => '_',
            };
        }
    }
}



// makes all squares return to their original color
pub fn refresh_board_color(board: &mut Board){
    for (row_idx, row) in board.iter_mut().enumerate() {
        for (col_idx, square) in row.iter_mut().enumerate() {
            square.color = BOARD_LAYOUT[row_idx][col_idx].color;
        }
    }
}