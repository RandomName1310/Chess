mod moves;
use moves::*;

mod board;
use board::*;

use macroquad::prelude::*;

const BOARD_SIZE: usize = 8;
const SQUARE_SIZE: f32 = 80.0;

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
fn draw_board(board: &mut Board){
    let mut is_white: bool = false;

    // calculate position based on screen size
    let board_px = BOARD_SIZE as f32 * SQUARE_SIZE;
    let x_offset = (screen_width() - board_px) / 2.0;
    let y_offset = (screen_height() - board_px) / 2.0;

    for x in 0..BOARD_SIZE {
        is_white = !is_white;
        for y in 0..BOARD_SIZE {
            let piece: &str = board[y][x];

            let pos_x = x_offset + x as f32 * SQUARE_SIZE;
            let pos_y = y_offset + y as f32 * SQUARE_SIZE;

            if is_white{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, WHITE)} 
            else{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, BLACK)} 
            draw_text(&format!("{}", piece), pos_x + 30.0, pos_y + 60.0, 50.0, RED);

            is_white = !is_white;
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
        clear_background(GRAY);
        draw_board(&mut board);
        move_piece(&mut board, (1, 2));
        next_frame().await;
    }
}
