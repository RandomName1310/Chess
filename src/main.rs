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

fn get_board_offset() -> (f32, f32){
    // calculate position based on screen size
    let board_px = BOARD_SIZE as f32 * SQUARE_SIZE;
    let x_offset = (screen_width() - board_px) / 2.0;
    let y_offset = (screen_height() - board_px) / 2.0;

    (x_offset, y_offset)
}

fn get_selected_square() -> (usize, usize){
    let mouse_pos: (f32, f32) = mouse_position();
    let board_offset: (f32, f32) = get_board_offset();

    // calculate specific square coord
    let square_selected: (usize, usize) = (((mouse_pos.0 - board_offset.0) / SQUARE_SIZE) as usize, 
                                           ((mouse_pos.1 - board_offset.1) / SQUARE_SIZE) as usize);

    square_selected
}

// draw the board with the pieces on screen
fn draw_board(board: &mut Board, pieces: &mut Board){
    let board_offset: (f32, f32) = get_board_offset();

    // set pieces and squares
    for x in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            let piece: &str = pieces[y][x];

            let pos_x = board_offset.0 + x as f32 * SQUARE_SIZE;
            let pos_y = board_offset.1 + y as f32 * SQUARE_SIZE;

            if board[y][x] == "W"{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, WHITE)} 
            else if board[y][x] == "B"{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, BLACK)}
            else if board[y][x] == "R"{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, RED)} 
            draw_text(&format!("{}", piece), pos_x + 30.0, pos_y + 60.0, 50.0, RED);
        }
    }
}

fn move_piece(board: &mut Board, _pieces: &mut Board, _coord: (i8, i8)) {
    refresh_board(board);
    let (square_x,square_y): (usize, usize) = get_selected_square();
    board[square_y][square_x] = "R";
}

#[macroquad::main("Jogo de Xadrez")]
async fn main(){
    let mut board: Board = BOARD_LAYOUT;
    let mut pieces: Board = PIECE_LAYOUT;

    loop {
        clear_background(GRAY);
        if is_mouse_button_pressed(MouseButton::Left) {
            move_piece(&mut board, &mut pieces, (1, 2));
        }
        draw_board(&mut board, &mut pieces);
        next_frame().await;
    }
}
