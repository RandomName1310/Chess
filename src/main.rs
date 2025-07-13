mod moves;
use moves::*;

mod board;
use board::*;

use macroquad::prelude::*;

fn get_board_offset() -> (f32, f32){
    // calculate position based on screen size
    let board_px = BOARD_SIZE as f32 * SQUARE_SIZE;
    let x_offset = (screen_width() - board_px) / 2.0;
    let y_offset = (screen_height() - board_px) / 2.0;

    (x_offset, y_offset)
}

fn color_pieces_path(piece: Piece, board: &mut Board, pieces: &mut Board){
    // unwrap moveset
    let path: Moveset = match piece.piece_type {
        PieceType::Pawn(m)   => m,
        PieceType::Rook(m)   => m,
        PieceType::Knight(m) => m,
        PieceType::Bishop(m) => m,
        PieceType::Queen(m)  => m,
        PieceType::King(m)   => m,
        PieceType::Empty     => {
            println!("Empty piece has no moveset.");
            return;
        }
    };

    for offset in path{
        let mut pos_x = piece.x as isize;
        let mut pos_y = piece.y as isize;

        for _ in 0..offset.repeats{
            pos_x += offset.dx as isize;
            pos_y += offset.dy as isize;

            // cover cases
            if pos_x < 0 || pos_x >= BOARD_SIZE as isize || pos_y < 0 || pos_y >= BOARD_SIZE as isize {
                break;
            }
            if piece.piece_type != PieceType::Knight(KNIGHT_MOVES) && pieces[pos_y as usize][pos_x as usize] != ' '{
                break;
            }

            board[pos_y as usize][pos_x as usize] = 'R';
        }
    }
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
fn select_piece(board: &mut Board, pieces: &mut Board) -> Piece {
    let (square_x, square_y) = get_selected_square();
    let piece_ch: char = pieces[square_y][square_x];

    let color = if piece_ch.is_uppercase() {PieceColor::Black } else {PieceColor::White };

    let piece_type = match piece_ch.to_ascii_lowercase() {
        'p' => {
            let moveset = if (color == PieceColor::White && square_y == 6)
                       || (color == PieceColor::Black && square_y == 1) {

                if color == PieceColor::Black {PAWN_BLACK_DOUBLE_FORWARD}
                else                     {PAWN_WHITE_DOUBLE_FORWARD}
            } else { 
                if color == PieceColor::Black {PAWN_BLACK_MOVES}
                else                     {PAWN_WHITE_MOVES}
            };
            PieceType::Pawn(moveset) 
        }
        'r' => PieceType::Rook(ROOK_MOVES),
        'b' => PieceType::Bishop(BISHOP_MOVES),
        'n' => PieceType::Knight(KNIGHT_MOVES),
        'q' => PieceType::Queen(QUEEN_MOVES),
        'k' => PieceType::King(KING_MOVES),
        ' ' => PieceType::Empty,
        _   => panic!("unknown symbol {}", piece_ch),
    };

    // remove as casas vermelhas da seleção anterior
    refresh_board(board);

    let piece = Piece {
        x: square_x,
        y: square_y,
        piece_type,
        color,
    };

    // colore o caminho possível da peça recém‑selecionada
    color_pieces_path(piece, board, pieces);

    piece
}

// move a piece from the board
fn move_piece(board: &mut Board, pieces: &mut Board, last_piece_data: Piece){
    let (square_x,square_y): (usize, usize) = get_selected_square();
    let Piece{x: piece_x, y: piece_y, piece_type, color}: Piece = last_piece_data;

   
    if board[square_y][square_x] == 'R'{
        pieces[piece_y][piece_x] = ' ';
        pieces[square_y][square_x] = match piece_type{
            PieceType::Pawn(_) => if color == PieceColor::Black{'P'}else{'p'},
            PieceType::Bishop(_) => if color == PieceColor::Black{'B'}else{'b'},
            PieceType::Knight(_) => if color == PieceColor::Black{'N'}else{'n'},
            PieceType::Rook(_) => if color == PieceColor::Black{'R'}else{'r'},
            PieceType::Queen(_) => if color == PieceColor::Black{'Q'}else{'q'},
            PieceType::King(_) => if color == PieceColor::Black{'K'}else{'k'},
            PieceType::Empty =>  ' ',
        }
    }

    // remove red squares from previous calls
    refresh_board(board);
}

#[macroquad::main("Jogo de Xadrez")]
async fn main(){
    let mut board: Board = BOARD_LAYOUT;
    let mut pieces: Board = PIECE_LAYOUT;

    let mut selected_piece_data: Piece = {Piece{x: 0, y: 0, piece_type: PieceType::Empty, color: PieceColor::White}};
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
