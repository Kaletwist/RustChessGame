mod pieces;
mod moveset;
use std::usize;

use minifb::{Key,Window,WindowOptions};
use pieces::ChessPieces;

const WIDTH: usize = 640;
const HEIGHT: usize = 700;
fn main() {
    let mut turn: i8 = 1;
    let mut board: Vec<Vec<u32>> = vec![vec![0; WIDTH]; HEIGHT];
    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];
    let mut square_contains: Vec<Vec<i8>> = vec![vec![0; 8]; 8];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut pieces: Vec<pieces::ChessPieces> = pieces::ChessPieces::init();
    update_pieces(&mut pieces, &mut square_contains);
    for (i, a) in board.iter_mut().enumerate() {
        for (j, b) in a.iter_mut().enumerate() {
            if i %160 < 80 && j % 160 < 80 && i < 640  || (i % 160 >= 80 && j % 160 >= 80) {
                *b = 0x006600;
            }else  if i < 640{
                *b = 0xd1ccba;
            }else {
                *b = 0x97572B;
            }
        }
    }
    for mut piece in pieces.iter_mut() {
        place_piece(&mut piece, &mut board);
    }
    
    // Limit to max ~60 fps update rate
    window.set_target_fps(60);
    let mut mousedown: bool = false;
    let mut mouse_up = false;
    let mut check_piece: Vec<usize>;
    let mut last_pos: (i32, i32) = (0,0);
    let mut this_move: (i32, i32) = (9,9);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        buffer = to_window_buffer(&board);
        mousedown =  window.get_mouse_down(minifb::MouseButton::Left);

        if let Some((xmouse , ymouse)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {

            highlight(&mut buffer, &xmouse, &ymouse);
            check_piece = in_check(&pieces, &turn);

            if !check_piece.is_empty() {
                update_pieces_check(&mut pieces, &mut square_contains, &turn);
                if check_mate(&pieces, &turn) {
                    if turn == 1 {
                        println!("Black Wins");
                    }else {
                        println!("White Wins");
                    }
                    return;
                }
            }else if check_mate(&pieces, &turn) {
                println!("Stale Mate");
                return;
            }

            if mousedown {
                let mut x = xmouse.floor() as i32;
                let mut y: i32 = ymouse.floor() as i32;
                let mut piece_index = 0;
                x = x / 80;
                y = y / 80;

                if square_contains[x as usize][y as usize] == turn {
                    for piece in pieces.iter() {
                        if piece.xpos == x && piece.ypos == y {
                            break;
                        }
                        piece_index += 1;
                    }

                    en_passant(&this_move, &last_pos, &turn, &mut pieces);
                    if pieces[piece_index].role == "King" {castle(&square_contains, &mut pieces, &turn, &piece_index)};

                    if !pieces[piece_index].views.is_empty(){

                        for (mut xpos, mut ypos) in pieces[piece_index].views.clone() {
                            xpos *= 80;
                            ypos *= 80;
                            highlight(&mut buffer, &(xpos as f32), &(ypos as f32));
                        }

                        let mut mouse_up = false;
                        'outer: loop {
                            update_pieces_check(&mut pieces, &mut square_contains, &turn);
                            en_passant(&this_move, &last_pos, &turn, &mut pieces);
                            if pieces[piece_index].role == "King" {castle(&square_contains, &mut pieces, &turn, &piece_index)};
                            mousedown = window.get_mouse_down(minifb::MouseButton::Left);

                            if !mouse_up {mouse_up = !window.get_mouse_down(minifb::MouseButton::Left);}

                            for (xpos, ypos) in pieces[piece_index].views.clone() {
                                if let Some((mut xi, mut yi)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                                    xi = (xi / 80.0).floor();
                                    yi = (yi / 80.0).floor(); 

                                    if !mousedown || !mouse_up || xpos != (xi as i32) || ypos != (yi as i32) {continue;}
                                    this_move = (xi as i32, yi as i32);
                                    last_pos = (pieces[piece_index].xpos, pieces[piece_index].ypos);
                                    if (last_pos.0 - this_move.0).abs() > 1 && pieces[piece_index].role == "King" {
                                        if last_pos.0 - this_move.0 > 0 {
                                            for piece in pieces.iter_mut() {
                                                if piece.role == "Rook" && piece.xpos == 0 && piece.ypos == this_move.1 {
                                                    pickup_piece(piece, &mut board);
                                                    piece.xpos = this_move.0 + 1;
                                                    place_piece(piece, &mut board);
                                                }
                                            }   
                                        }else {
                                            for piece in pieces.iter_mut() {
                                                if piece.role == "Rook" && piece.xpos == 7 && piece.ypos == this_move.1 {
                                                    pickup_piece(piece, &mut board);
                                                    piece.xpos = this_move.0 - 1;
                                                    place_piece(piece, &mut board);
                                                }
                                            }
                                        }
                                    } 
                                    
                                    pickup_piece(&pieces[piece_index], &mut board);
                                    let mut captured_piece_index: usize = usize::MAX;

                                    if square_contains[xpos as usize][ypos as usize] != turn && square_contains[xpos as  usize][ypos as usize] != 0 {
                                        captured_piece_index = capture(&xpos, &ypos, &mut pieces, &mut board);
                                    }else if pieces[piece_index].role == "Pawn" && pieces[piece_index].xpos != this_move.0 {
                                        if square_contains[this_move.0 as usize][(this_move.1 - 1) as usize] != 0 {
                                            captured_piece_index = capture(&this_move.0, &(this_move.1 - 1), &mut pieces, &mut board);
                                        }else if square_contains[this_move.0 as usize][(this_move.1 + 1) as usize] != 0 {
                                            captured_piece_index = capture(&this_move.0, &(this_move.1 + 1), &mut pieces, &mut board);
                                        }
                                    }

                                    if captured_piece_index < piece_index {piece_index-=1;}

                                    pieces[piece_index].xpos = this_move.0;
                                    pieces[piece_index].ypos = this_move.1;

                                    update_pieces(&mut pieces, &mut square_contains);
                                    place_piece(&mut pieces[piece_index], &mut board);
                                        
                                    pieces[piece_index].moved = true;
                                    if turn == 1 {turn = 2}else {turn = 1}
                                        
                                    break 'outer;
                                }
                            }
                            if (mousedown && mouse_up){
                                mousedown = false;
                                mouse_up = false;
                                break 'outer;
                            }
                            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                        }
                    }
                }
            }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}
fn castle(square_contains: & Vec<Vec<i8>>, pieces: & mut Vec<ChessPieces>, turn: &i8, king_index: &usize) {
    let mut long_castle: bool = true;
    let mut short_castle: bool = true;
    let ypos: usize = pieces[*king_index].ypos as usize;
    let king_colour: String = pieces[*king_index].colour.clone();
    'outer: for x in 1..3 {
        if square_contains[x][ypos] != 0 {
            short_castle = false;
            break;
        }
        for piece in pieces.iter() {
            for view in piece.views.iter() {
                if (x as i32, ypos as i32) == *view && piece.colour != king_colour {
                    short_castle = false;
                    break 'outer;
                }
            }
        }
    }
    'outer1: for x in 4..6 {
        if square_contains[x][ypos] != 0 {
            println!("1");
            long_castle = false;
            break;
        }
        for piece in pieces.iter() {
            for view in piece.views.iter() {
                if (x as i32, ypos as i32) == *view && piece.colour != king_colour {
                    long_castle = false;
                    break 'outer1;
                }
            }
        }
    }
    if short_castle {
        pieces[*king_index].views.push((1, ypos as i32));
    }
    println!("{}", long_castle);
    if long_castle {
        pieces[*king_index].views.push((5, ypos as i32));
    }
}
fn en_passant(last_move_to: &(i32,i32), last_move_from: &(i32,i32), turn: &i8, pieces: &mut Vec<ChessPieces>) {
    let mut en_passant_possible = false;
    for piece in pieces.iter() {
        if *last_move_to == (piece.xpos, piece.ypos) {
            if last_move_to.0 == last_move_from.0 && (last_move_to.1 - last_move_from.1).abs() == 2  && piece.role == "Pawn"{
                en_passant_possible = true;
            }
        }
    }
    if !en_passant_possible {return;}
    for piece in pieces {
        if piece.ypos == last_move_to.1 && (piece.xpos == (last_move_to.0 - 1) || piece.xpos == (last_move_to.0 + 1) && piece.role == "Pawn") {
            if *turn == 1 {
                piece.views.push((last_move_to.0 , last_move_to.1 - 1));
            }else {
                piece.views.push((last_move_to.0 , last_move_to.1 + 1));
            }
        }
    }
}
fn check_mate(pieces: & Vec<ChessPieces>, turn: &i8) -> bool {
    let mut string = "";
    if *turn == 1 {
        string = "White";
    }else {
        string = "Black";
    }
    for piece in pieces {
        if !piece.views.is_empty() && piece.colour == string{
            return false;
        }
    }
    return true;
}
fn update_pieces_check(pieces: &mut Vec<ChessPieces>, square_contains: &mut Vec<Vec<i8>>, turn: &i8) {
    for x in square_contains.iter_mut() {
        for y in x {
            *y = 0;
        }
    }
    for piece in pieces.iter_mut() {
        let tempx = piece.xpos as usize;
        let tempy = piece.ypos as usize;
        let c: i8;
        if piece.colour == "Black" {c = 2}
        else {c = 1}
        square_contains[tempx][tempy] = c;
    }
    let mut check_pieces = pieces.clone();
    let mut current_piece: usize = 0;
    for piece in pieces.iter_mut() {
        piece.set_views(&square_contains);
        if (piece.colour == "Black" && *turn == 2) || (piece.colour == "White" && *turn == 1) {
            let mut to_remove = Vec::new();
            for (i, (xmove,ymove)) in piece.views.iter_mut().enumerate() {
                let tempx = piece.xpos;
                let tempy = piece.ypos;
                check_pieces[current_piece].xpos = *xmove;
                check_pieces[current_piece].ypos = *ymove;
                update_pieces(&mut check_pieces, square_contains);
                if !in_check(&check_pieces, turn).is_empty() {
                    to_remove.push(i);
                }
                check_pieces[current_piece].xpos = tempx;
                check_pieces[current_piece].ypos = tempy;
            }
            to_remove.sort_by(|a, b| b.cmp(a));
            for &i in to_remove.iter() {
                piece.views.remove(i);
                
            }
        }
        current_piece +=1;
    }
}
fn in_check(pieces: &Vec<ChessPieces>, turn: &i8) -> Vec<usize> {
    let king_x: i32;
    let king_y: i32;
    let king_index: usize;
    let mut checking_pieces: Vec<usize> = Vec::new();
    let mut piece_index: usize = 0;
    if *turn == 1 {
        king_index = 0;
    }else {
        king_index = 16;
    }
    king_x = pieces[king_index].xpos;
    king_y = pieces[king_index].ypos;
    for piece in pieces.iter() {
        for (viewx,viewy) in piece.views.iter() {
            if *viewx == king_x && *viewy == king_y {
                checking_pieces.push(piece_index);
            }
        }
        piece_index += 1;
    }
    return checking_pieces;
}
fn capture(x: &i32, y: &i32, pieces: &mut Vec<ChessPieces>, board: &mut Vec<Vec<u32>>) -> usize{
    let mut captured_piece_index: usize = 0;
    for piece in pieces.iter_mut() {
        if piece.xpos == *x && piece.ypos == *y {
            break;
        }
        captured_piece_index += 1;
    }
    pickup_piece(&pieces[captured_piece_index], board);
    pieces.remove(captured_piece_index);
    return captured_piece_index;
}
fn pickup_piece(piece: &ChessPieces, board: &mut Vec<Vec<u32>>) {
    let mut x = piece.xpos * 80;
    let mut y = piece.ypos * 80;
    println!("{},{}", x, y);
    for i in y..(y + 80) {
        for j in x..(x + 80) {
            if i %160 < 80 && j % 160 < 80 && i < 640  || (i % 160 >= 80 && j % 160 >= 80) {
                board[i as usize][j as usize] = 0x006600;
            }else  if i < 640{
                board[i as usize][j as usize] = 0xd1ccba;
            }else {
                board[i as usize][j as usize] = 0x97572B;
            }
        }
    }
}
fn place_piece(piece: &mut ChessPieces, board: &mut Vec<Vec<u32>>) {
    for (_row, a) in piece.pic.enumerate_rows() {
        for (mut x, mut y, &rgb) in a {
            x = x + (piece.xpos * 80) as u32;
            y = y + (piece.ypos * 80) as u32;
            let temp: u32 = ((rgb[0] as u32) << 16) + ((rgb[1] as u32) << 8) + ((rgb[2] as u32));
            if rgb[3]!=0 {
                board[y as usize][x as usize] = temp;
            }
        }
    }
}
fn update_pieces(pieces: &mut Vec<ChessPieces>, square_contains: &mut Vec<Vec<i8>>) {
    for x in square_contains.iter_mut() {
        for y in x {
            *y = 0;
        }
    }
    for piece in pieces.iter_mut() {
        let tempx = piece.xpos as usize;
        let tempy = piece.ypos as usize;
        let c: i8;
        if piece.colour == "Black" {c = 2}
        else {c = 1}
        square_contains[tempx][tempy] = c;
    }
    for piece in pieces.iter_mut() {
        piece.set_views(&square_contains);
    }
}
fn highlight(buffer: &mut Vec<u32>, xmouse: &f32, ymouse: &f32) {
    if *ymouse < 640.0 {
        let mut temp = (xmouse / 80.0).floor() as usize;
        let xi = temp * 80;
        temp = (ymouse / 80.0).floor() as usize;
        let yi = temp * 80;
        for yi in yi..yi+80 {
            for xi in xi..xi+80 {
                if buffer[yi * WIDTH + xi] < 0xd1ccbb {
                    buffer[yi * WIDTH + xi] += 0x222222;
                }    
            }
        }
    }
}

fn to_window_buffer(board: &Vec<Vec<u32>>) -> Vec<u32>{
    let mut result: Vec<u32> = vec![0; WIDTH*HEIGHT];
    result.reserve(WIDTH*HEIGHT);
    for (i, a) in board.iter().enumerate() {
        for (j, b) in a.iter().enumerate() {
            result[i*640 + j] = *b;
        }
    }
    return result;
}