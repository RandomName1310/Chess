mod moves;
use moves::*;

mod board;
use board::*;

use macroquad::prelude::*;

const BOARD_SIZE: usize = 8;
const X_OFFSET: f32 = 100.0;
const Y_OFFSET: f32 = 100.0;

type Board = [[&'static str; 8]; 8];

#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color,
    moves_move: &'static [Offset],
    moves_capture: &'static [Offset],
}

// draw the board with the pieces on screen
fn draw_board(board: &mut Board) {
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let piece: &str = board[y][x];
            draw_text(&format!("{}", piece), 20.0 + X_OFFSET * x as f32, 20.0 + Y_OFFSET * y as f32, 20.0, RED);
        }
    }
}

fn move_piece(_board: &mut Board, _coord: (i8, i8)) {
    // later...
}

#[macroquad::main("Jogo de Xadrez")]
async fn main(){
    let mut board: Board = create_board();

    loop {
        clear_background(WHITE);
        draw_board(&mut board);
        move_piece(&mut board, (1, 2));
        next_frame().await;
    }
}
