mod moves;
use moves::*;

const BOARD_SIZE: usize = 8;

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
    moves_move: &'static[Offset],
    moves_capture: &'static[Offset]
}

type Board = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]; // type makes something equivalent
// example: Board now is equal to [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]. Makes it simpler


// set pieces
fn init_board() -> Board{
    let mut board: Board = [[None; BOARD_SIZE]; BOARD_SIZE];

    // place pawns
    for x in 0..BOARD_SIZE{
        board[1][x] = Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        board[6][x] = Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
    }

    // place rooks
    board[7][7] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
    board[7][0] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
    board[0][0] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });
    board[0][7] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });

    // place knight
    board[7][6] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
    board[7][1] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
    board[0][6] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });
    board[0][1] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });

    board
}

// create board tiles
fn create_board(board: &Board){
    const X_OFFSET: f32 = 100.0;
    const Y_OFFSET: f32 = 100.0;
    let is_white: bool = true;

    for x in 0..BOARD_SIZE{
        for y in 0..BOARD_SIZE{
            let piece: Option<Piece> = board[y][x];
            let letter: if let Some(piece) = piece {
                match piece.piece_type{
                    PieceType::Pawn => letter = String::from("P");
                    PieceType::Rook => letter = String::from("R");
                    PieceType::Knight => letter = String::from("N");
                    PieceType::Bishop => letter = String::from("B");
                    PieceType::King => letter = String::from("K");
                    PieceType::Queen => letter = String::from("Q");
                    _ => letter = String::from("ERR");
                }
            }

            draw_text(&format!("{}", letter), 20.0 + X_OFFSET * x, Y_OFFSET * y, 20.0, RED);
        }
    }
}

fn move_piece(board: &mut Board, coord: (i8, i8)){

}

#[macroquad::main("Jogo 2D Simples")]
async fn main() {
    let mut board: Board = init_board();
    create_board(&board);
    move_piece(&mut board, (1, 2));
}