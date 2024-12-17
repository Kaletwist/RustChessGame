use std::sync::{Arc, Mutex};

use crate::{board::Board, boardwindow::WindowFrame, piece::{update_views, ChessPiece, Colour}, squares::Square};

pub enum State {
    CHECKMATE,
    STALEMATE,
    INPLAY
}
pub fn inplay(board: &mut Board, pieces: &mut Vec<Vec<Option<ChessPiece>>>, last_move: &((i32, i32), (i32, i32)), turn: &mut Colour, x: usize, y: usize, window: &mut WindowFrame) -> ((i32,i32),(i32,i32)) {
    let selected_piece_temp = pieces[y][x].clone();
    if selected_piece_temp.is_none() {return *last_move;}

    let mut selected_piece = selected_piece_temp.unwrap();
    if selected_piece.colour != *turn {return *last_move}
    
    update_views(pieces, turn, &last_move);
    
    for (xt,yt) in selected_piece.get_moves() {
        board.board_highlight(yt, xt, true);
    }

    window.board_to_window(board);
    let (newx,newy) = get_mouse_click(window, board);
    println!("{}, {}", newx, newy);
    let mut legal_move = false;

    for (xt,yt) in selected_piece.moveset.clone() {
        if (newx as usize,newy as usize) == (xt,yt) {
            legal_move = true;
        }
    }

    if !legal_move {return *last_move;}
    let new_move: ((i32, i32), (i32, i32)) = ((x as i32,y as i32),(newx,newy));
    if newy < 0 {return *last_move;}

    if pieces[newy as usize][newx as usize].is_some() {
        board.board[newy as usize][newx as usize].pickup_piece();
    }
    Board::place_piece(&mut board.board, newy as usize, newx as usize, &Some(selected_piece.clone()));
    pieces[newy as usize][newx as usize] = Some(selected_piece.clone());
    board.board[x][y].pickup_piece();
    pieces[y][x] = None;
    pieces[newy as usize][newx as usize].as_mut().unwrap().moved = true;
    if *turn == Colour::WHITE {*turn = Colour::BLACK;}
    else {*turn = Colour::WHITE;}
    
    for (xt,yt) in selected_piece.get_moves() {
        board.board_highlight(xt, yt, false);
    }
    return new_move;
}

fn get_mouse_click(window: &mut WindowFrame, board: &mut Board) -> (i32, i32) {
    while !window.window_frame.get_mouse_down(minifb::MouseButton::Left) {
        window.board_to_window(board);
    }
    if let Some((xmouse, ymouse)) = window.window_frame.get_mouse_pos(minifb::MouseMode::Clamp) {
        return ((xmouse/80.0) as i32, ((ymouse- 60.0)/80.0) as i32);
    }
    return (9,9);
}
