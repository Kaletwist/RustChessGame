use crate::{piece::{self, ChessPiece, Colour, Piece}, squares::Square};
use image::{imageops, open, DynamicImage, ImageBuffer, Rgba};
use strum::IntoEnumIterator; 

pub struct Board {
    pub board: Vec<Vec<Square>>
}

impl Board {
    pub fn new() -> Board {
        let mut temp_board = Vec::new();
        for y in 0..8 {
            let mut row = Vec::new();
            for x in 0..8 {
                row.push(Square::new(x,y));
            }
            temp_board.push(row);
        }
        Board {board: temp_board}
    }
    pub fn place_piece (board: &mut Vec<Vec<Square>>, x: usize, y: usize, piece: &Option<ChessPiece>) {
        board[y][x].piece = piece.clone();
        board[y][x].place_piece();
    }
    pub fn pickup_piece (board: &mut Vec<Vec<Square>>, x: usize, y: usize) {
        board[y][x].piece = None;
        board[y][x].pickup_piece();
    }
    pub fn board_highlight(&mut self, x: usize, y: usize, on: bool) {
        if on {self.board[y][x].highlight()}
        else {self.board[y][x].undo_highlight()}
    }
}
