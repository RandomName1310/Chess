use crate::Board;

pub const BOARD_LAYOUT: [[&'static str; 8]; 8] = 
    [
        ["B", "W", "B", "W", "B", "W", "B", "W"], // row 0 (top)
        ["W", "B", "W", "B", "W", "B", "W", "B"], // row 1
        ["B", "W", "B", "W", "B", "W", "B", "W"], // row 2
        ["W", "B", "W", "B", "W", "B", "W", "B"], // row 3
        ["B", "W", "B", "W", "B", "W", "B", "W"], // row 4
        ["W", "B", "W", "B", "W", "B", "W", "B"], // row 5
        ["B", "W", "B", "W", "B", "W", "B", "W"], // row 6
        ["W", "B", "W", "B", "W", "B", "W", "B"], // row 7 (bottom)
    ];

pub const PIECE_LAYOUT: [[&'static str; 8]; 8] =
    [
        // black
        ["R", "N", "B", "Q", "K", "B", "N", "R"],
        ["P", "P", "P", "P", "P", "P", "P", "P"],

        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],

        // white
        ["p", "p", "p", "p", "p", "p", "p", "p"],
        ["r", "n", "b", "q", "k", "b", "n", "r"],
    ];

// makes all squares return to their original colors
pub fn refresh_board(board: &mut Board){
    *board = BOARD_LAYOUT;
}