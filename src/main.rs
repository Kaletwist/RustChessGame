mod pieces;
mod moveset;
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
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = to_window_buffer(&board);
        mousedown =  window.get_mouse_down(minifb::MouseButton::Left);
        if let Some((xmouse , ymouse)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            highlight(&mut buffer, &xmouse, &ymouse);
            if mousedown {
                let mut x = xmouse.floor() as i32;
                let mut y: i32 = ymouse.floor() as i32;
                let mut selected_piece: &ChessPieces = &pieces[0];
                let mut this_move: (i32, i32) = (9,9);
                let mut piece_index = 0;
                x = x / 80;
                y = y / 80;
                if square_contains[x as usize][y as usize] == turn {
                    for piece in pieces.iter() {
                        if piece.xpos == x && piece.ypos == y {
                            selected_piece = piece;
                            break;
                        }
                        piece_index += 1;
                    }
                    for (mut xpos, mut ypos) in selected_piece.views.clone() {
                        xpos *= 80;
                        ypos *= 80;
                        highlight(&mut buffer, &(xpos as f32), &(ypos as f32));
                    }
                    let mut mouse_up = false;
                    'outer: loop {
                        mousedown = window.get_mouse_down(minifb::MouseButton::Left);
                        if !mouse_up {mouse_up = !window.get_mouse_down(minifb::MouseButton::Left);}
                        for (xpos, ypos) in selected_piece.views.clone() {
                            if let Some((mut xi, mut yi)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
                                xi = (xi / 80.0).floor();
                                yi = (yi / 80.0).floor(); 
                                if mousedown && mouse_up && xpos == (xi as i32) && ypos == (yi as i32) {
                                    this_move = (xi as i32, yi as i32);
                                    mousedown = false;
                                    mouse_up = false;
                                    pickup_piece(&pieces[piece_index], &mut board);
                                    pieces[piece_index].xpos = this_move.0;
                                    pieces[piece_index].ypos = this_move.1;
                                    update_pieces(&mut pieces, &mut square_contains);
                                    place_piece(&mut pieces[piece_index], &mut board);
                                    if turn == 1 {turn = 2}else {turn = 1}
                                    break 'outer;
                                }else if mousedown && mouse_up {
                                    mousedown = false;
                                    mouse_up = false;
                                    break 'outer;
                                }
                            }
                        }

                        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                    }
                }
            }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}
fn pickup_piece(piece: &ChessPieces, board: &mut Vec<Vec<u32>>) {
    let mut x = piece.xpos * 80;
    let mut y = piece.ypos * 80;
    println!("{},{}", x, y);
    for i in y..(y + 81) {
        for j in x..(x + 81) {
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