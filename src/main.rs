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
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = to_window_buffer(&board);
        mousedown =  window.get_mouse_down(minifb::MouseButton::Left);
        if let Some((xmouse , ymouse)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            highlight(&mut buffer, &xmouse, &ymouse);
            check_piece = in_check(&pieces, &turn);
            if !check_piece.is_empty() {
                update_pieces_check(&mut pieces, &mut square_contains, &turn, check_piece);
                if check_mate(&pieces, &turn) {
                    if turn == 1 {
                        println!("Black Wins");
                    }else {
                        println!("White Wins");
                    }
                    return;
                }
            }
            if mousedown {
                let mut x = xmouse.floor() as i32;
                let mut y: i32 = ymouse.floor() as i32;
                let mut this_move: (i32, i32) = (9,9);
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
                    if !pieces[piece_index].views.is_empty(){
                        for (mut xpos, mut ypos) in pieces[piece_index].views.clone() {
                            xpos *= 80;
                            ypos *= 80;
                            highlight(&mut buffer, &(xpos as f32), &(ypos as f32));
                        }
                        let mut mouse_up = false;
                        'outer: loop {
                            mousedown = window.get_mouse_down(minifb::MouseButton::Left);
                            if !mouse_up {mouse_up = !window.get_mouse_down(minifb::MouseButton::Left);}
                            for (xpos, ypos) in pieces[piece_index].views.clone() {
                                if let Some((mut xi, mut yi)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                                    xi = (xi / 80.0).floor();
                                    yi = (yi / 80.0).floor(); 
                                    if mousedown && mouse_up && xpos == (xi as i32) && ypos == (yi as i32) {
                                        this_move = (xi as i32, yi as i32);
                                        pickup_piece(&pieces[piece_index], &mut board);
                                        let mut captured_piece_index: usize = usize::MAX;
                                        if square_contains[xpos as usize][ypos as usize] != turn && square_contains[xpos as  usize][ypos as usize] != 0 {
                                            captured_piece_index = capture(&xpos, &ypos, &mut pieces, &mut board);
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
fn update_pieces_check(pieces: &mut Vec<ChessPieces>, square_contains: &mut Vec<Vec<i8>>, turn: &i8, checking_pieces: Vec<usize>) {
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