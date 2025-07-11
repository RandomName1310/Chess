mod moves;
use moves::*;

mod board;
use board::*;

use macroquad::prelude::*;

const BOARD_SIZE: usize = 8;
const SQUARE_SIZE: f32 = 80.0;

type Board = [[char; 8]; 8];

#[derive(Copy, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
            let piece: char = pieces[y][x];

            let pos_x = board_offset.0 + x as f32 * SQUARE_SIZE;
            let pos_y = board_offset.1 + y as f32 * SQUARE_SIZE;

            if board[y][x] == 'W'{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, WHITE)} 
            else if board[y][x] == 'B'{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, BLACK)}
            else if board[y][x] == 'R'{draw_rectangle(pos_x, pos_y, SQUARE_SIZE, SQUARE_SIZE, RED)} 
            draw_text(&format!("{}", piece), pos_x + 30.0, pos_y + 60.0, 50.0, GRAY);
        }
    }
}

// select a piece from the board
fn select_piece(board: &mut Board, pieces: &mut Board) -> (usize, usize, PieceType, Color){
    let (square_x,square_y): (usize, usize) = get_selected_square();
    let piece: char = pieces[square_y][square_x];

    let color: Color = if piece.is_uppercase() { Color::Black } else { Color::White };
    let piece_type = match piece.to_ascii_lowercase(){
        'p' => PieceType::Pawn,
        'r' => PieceType::Rook,
        'b' => PieceType::Bishop,
        'n' => PieceType::Knight,
        'q' => PieceType::Queen,
        'k' => PieceType::King,
        ' ' => PieceType::Empty,
        _   => panic!("unknown symbol {}", piece),
    };

    // remove red squares from previous calls
    refresh_board(board);

    // create red square
    board[square_y][square_x] = 'R';
    (square_x,square_y, piece_type, color)
}

// move a piece from the board
fn move_piece(board: &mut Board, pieces: &mut Board, last_piece_data: (usize, usize, PieceType, Color)){
    let (square_x,square_y): (usize, usize) = get_selected_square();
    let (piece_x, piece_y, piece_type, color): (usize, usize, PieceType, Color) = last_piece_data;

    // remove red squares from previous calls
    refresh_board(board);

    pieces[piece_y][piece_x] = ' ';
    pieces[square_y][square_x] = match piece_type{
        PieceType::Pawn =>  if color == Color::Black{'P'}else{'p'},
        PieceType::Bishop =>  if color == Color::Black{'B'}else{'b'},
        PieceType::Knight =>  if color == Color::Black{'N'}else{'n'},
        PieceType::Rook =>  if color == Color::Black{'R'}else{'r'},
        PieceType::Queen =>  if color == Color::Black{'Q'}else{'q'},
        PieceType::King =>  if color == Color::Black{'K'}else{'k'},
        PieceType::Empty =>  ' ',
    }
}

#[macroquad::main("Jogo de Xadrez")]
async fn main(){
    let mut board: Board = BOARD_LAYOUT;
    let mut pieces: Board = PIECE_LAYOUT;

    let mut selected_piece_data: (usize, usize, PieceType, Color) = (0, 0, PieceType::Empty, Color::White);
    let mut is_selecting_piece: bool = true;

    loop {
        clear_background(GRAY);

        // select or move piece
        if is_mouse_button_pressed(MouseButton::Left){
            if is_selecting_piece {selected_piece_data = select_piece(&mut board, &mut pieces);
                                         is_selecting_piece = !is_selecting_piece}
            else if !is_selecting_piece {move_piece(&mut board, &mut pieces, selected_piece_data); 
                                         is_selecting_piece = !is_selecting_piece}
        }
        draw_board(&mut board, &mut pieces);
        next_frame().await;
    }
}
