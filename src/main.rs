use std::{borrow::Borrow, os::windows, process::exit, sync::{Arc, Mutex}};

use boardwindow::WindowFrame;
use board::Board;
use piece::{ChessPiece, Colour};
mod squares;
mod game_logic;
mod moveset;
mod boardwindow;
mod board;
mod piece;

fn main () {
    let mut turn: Colour = Colour::WHITE;
    let mut this_window: WindowFrame = boardwindow::WindowFrame::setup_window();

    let mut board: Board = Board::new();

    let mut last_move: ((i32, i32), (i32, i32)) = ((0,0), (0,0));
    
    let mut pieces: Vec<Vec<Option<ChessPiece>>>; 
    pieces = piece::init_pieces();
    for (y,row)  in pieces.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            if piece.is_none() {println!("{},{}, NONE", x, y); continue;}
            println!("{},{}: {:?}, {:?}", x, y, piece.as_ref().unwrap().role, piece.as_ref().unwrap().colour);
        }
    }

    for (x, row) in pieces.iter().enumerate() {
        for (y , piece) in row.iter().enumerate() {
            //if piece.is_none() {continue;}
            println!("{}, {}", x, y);
            Board::place_piece(&mut board.board, x, y, piece);
        }
    }

    let game_state: game_logic::State = game_logic::State::INPLAY;

    while this_window.is_open() {
        if this_window.window_frame.get_mouse_down(minifb::MouseButton::Left) {
            last_move = start_turn(&mut this_window, &mut board, &mut pieces, &game_state, &last_move, &mut turn);
        }
        this_window.board_to_window(&board);
    }
}

fn start_turn (this_window: &mut WindowFrame, game_board: &mut Board, pieces: &mut Vec<Vec<Option<ChessPiece>>>, game_state: &game_logic::State, last_move: &((i32, i32), (i32, i32)), turn: &mut Colour) -> ((i32,i32),(i32,i32)) {
    let xpos: usize;
    let ypos: usize;
    if let Some((mousex,mousey)) = this_window.window_frame.get_mouse_pos(minifb::MouseMode::Clamp) {
        xpos = (mousex/80.0) as usize;
        ypos = ((mousey- 60.0)/80.0) as usize;
    }else {return *last_move;}
    if ypos > 7 {
        return *last_move;
    }
    match game_state {
        game_logic::State::INPLAY =>  return game_logic::inplay(game_board, pieces, &last_move, turn, xpos, ypos, this_window),
        game_logic::State::CHECKMATE => game_over(turn),
        game_logic::State::STALEMATE => game_tied()
    };
    return *last_move;
}
fn game_over(turn: &Colour) {
    if *turn == Colour::WHITE {
        println!("Check Mate: Black Wins!");
    }else {
        println!("Check Mate: White Wins!");
    }
    exit(0);
}

fn game_tied() {
    println!("Stale Mate: Tie!");
    exit(0);
}
