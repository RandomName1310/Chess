use crate::moves::*;

pub const BOARD_SIZE: usize = 8;
pub const SQUARE_SIZE: f32 = 80.0;

pub type Board = [[char; 8]; 8];

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

pub const BOARD_LAYOUT: [[char; 8]; 8] = [
    ['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'], // row 0 (top)
    ['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'], // row 1
    ['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'], // row 2
    ['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'], // row 3
    ['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'], // row 4
    ['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'], // row 5
    ['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'], // row 6
    ['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'], // row 7 (bottom)
];

pub const PIECE_LAYOUT: [[char; 8]; 8] = [
    // black pieces
    ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
    ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],

    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],

    // white pieces
    ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
    ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
];



// makes all squares return to their original coll
pub fn refresh_board(board: &mut Board){
    *board = BOARD_LAYOUT;
}