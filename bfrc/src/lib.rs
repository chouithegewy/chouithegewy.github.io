mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Board {
    width: u8,
    height: u8,
    squares: [Piece; 64],
}

impl Board {
    fn get_index(&self, row: u8, column: u8) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    P,
    R,
    N,
    B,
    Q,
    K,
    None,
}

pub struct Position {
    rank: u8,
    file: char,
}

impl Position {
    fn get_row(&self) -> u8 {
        self.rank - 1
    }

    fn get_col(&self) -> char {
        char::from_digit(self.file.make_ascii_lowercase() - 1)
    }
}
// formula for flat board 8x8 ... board[0..63]
// index(row, column, board) = row * width(board) + column
// so, row 0, col 0, would eq board[0] aka a8
// ... row 0, 7, would eq board[7] aka h8


