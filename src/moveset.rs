use crate::pieces::ChessPieces;

pub fn king(piece: &ChessPieces, square_contains: &Vec<Vec<i8>>) -> Vec<(i32, i32)> {
    let mut move_set: Vec<(i32,i32)> = Vec::new();
    let x = piece.xpos;
    let y = piece.ypos;
    let mut c: i8 = 0;
    if piece.colour == "White"{c = 1}
    else if piece.colour == "Black" {c = 2}
    for xmove in x-1..x+2 {
        for ymove in y-1..y+2 {
            if is_on_board(xmove, ymove) && !(xmove == x && ymove == y) && square_contains[xmove as usize][ymove as usize] == c {
                move_set.push((xmove, ymove));
            }
        }
    }
    return move_set;
}
pub fn pawn (piece: &ChessPieces, square_contains: &Vec<Vec<i8>>) -> Vec<(i32, i32)> {
    let mut move_set: Vec<(i32,i32)> = Vec::new();
    let x = piece.xpos;
    let y = piece.ypos;
    if piece.colour == "White"{
        if is_on_board(x,y-2) && !piece.moved && square_contains[(x) as usize][(y-2) as usize] != 1{
            move_set.push((x, y - 2));
        }
        if is_on_board(x+1,y-1) && square_contains[(x+1) as usize][(y-1) as usize] == 2 {
            move_set.push((x + 1, y - 1));
        }
        if is_on_board(x-1,y+1) && square_contains[(x - 1) as usize][(y - 1) as usize] == 2 {
            move_set.push((x - 1, y + 1));
        }
        if is_on_board(x,y-1) && square_contains[(x) as usize][(y-1) as usize] == 0 {move_set.push((x, y - 1));}
    }
    else if piece.colour == "Black" {
        if is_on_board(x,y+2) && !piece.moved && square_contains[(x) as usize][(y + 2) as usize] != 2{
            move_set.push((x, y + 2));
        }
        if is_on_board(x+1,y+1) && square_contains[(x + 1) as usize][(y + 1) as usize] == 1 {
            move_set.push((x + 1, y +1));
        }
        if is_on_board(x-1,y+1) && square_contains[(x-1) as usize][(y+1) as usize] == 1 {
            move_set.push((x - 1, y + 1));
        }
        if is_on_board(x,y+1) && square_contains[(x) as usize][(y+1) as usize] == 0 {move_set.push((x, y + 1))};
    }
    return move_set;
}
pub fn knight(piece: &ChessPieces, square_contains: &Vec<Vec<i8>>) -> Vec<(i32, i32)> {
    let mut move_set: Vec<(i32,i32)> = Vec::new();
    let mut x = piece.xpos;
    let mut y = piece.ypos;
    let mut c: i8 = 0;
    if piece.colour == "White"{c = 1}
    else if piece.colour == "Black" {c = 2}
    x += 2;
    y+=1;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    y -= 2;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    y += 1;
    x -= 4;
    y+=1;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    y -= 2;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    x = piece.xpos;
    y = piece.ypos;

    y += 2;
    x+=1;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    
    x -= 2;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    
    x += 1;
    y -= 4;
    x +=1;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}

    x -= 2;
    if is_on_board(x, y) && square_contains[x as usize][y as usize] != c {move_set.push((x,y));}
    
    return move_set;
}
pub fn rook(piece: &ChessPieces, square_contains: &Vec<Vec<i8>>) -> Vec<(i32,i32)> {
    let mut move_set: Vec<(i32,i32)> = Vec::new();
    let mut c: i8 = 0;
    if piece.colour == "White"{c = 1}
    else if piece.colour == "Black" {c = 2}
    straight(piece.xpos, piece.ypos, &mut move_set, square_contains, c);
    
    return move_set;
}
pub fn queen(piece: &ChessPieces, square_contains: &Vec<Vec<i8>>) -> Vec<(i32,i32)> {
    let mut move_set: Vec<(i32,i32)> = Vec::new();
    let mut c: i8 = 0;
    if piece.colour == "White"{c = 1}
    else if piece.colour == "Black" {c = 2}
    straight(piece.xpos, piece.ypos, &mut move_set, square_contains, c);
    diagonal(piece.xpos, piece.ypos, &mut move_set, square_contains, c);
    
    return move_set;
}
pub fn bishop(piece: &ChessPieces, square_contains: &Vec<Vec<i8>>) -> Vec<(i32,i32)> {
    let mut move_set: Vec<(i32,i32)> = Vec::new();
    let mut c: i8 = 0;
    if piece.colour == "White"{c = 1}
    else if piece.colour == "Black" {c = 2}
    diagonal(piece.xpos, piece.ypos, &mut move_set, square_contains, c);
    
    return move_set;
}

fn straight(x: i32, y: i32, move_set: &mut Vec<(i32,i32)>, square_contains: &Vec<Vec<i8>>, c: i8) {
    let mut tempx = x;
    let mut tempy = y;
    while tempx < 8 {
        if tempx != x {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempx += 1;
    }
    tempx = x;
    while tempx >= 0 {
        if tempx != x {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempx -= 1;
    }
    tempx = x;
    while tempy < 8 {
        if tempy != y {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempy += 1;
    }
    tempy = y;
    while tempy >= 0 {
        if tempy != y {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempy -= 1;
    }
}
fn diagonal(x: i32, y: i32, move_set: &mut Vec<(i32,i32)>, square_contains: &Vec<Vec<i8>>, c: i8) {
    let mut tempx = x;
    let mut tempy = y;
    while tempx < 8 && tempy < 8 {
        if tempx != x {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempx += 1;
        tempy += 1;
    }
    tempx = x;
    tempy = y;
    while tempx >= 0 && tempy < 8 {
        if tempx != x {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempx -= 1;
        tempy += 1;
    }
    tempx = x;
    tempy = y;
    while tempx < 8 && tempy >= 0 {
        if tempy != y {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempx += 1;
        tempy -=1;
    }
    tempx = x;
    tempy = y;
    while tempx >= 0 && tempy >= 0 {
        if tempy != y {
            if square_contains[tempx as usize][tempy as usize] == c {break;}
            move_set.push((tempx, tempy));
            if square_contains[tempx as usize][tempy as usize] != 0 {break;}
        }
        tempx -= 1;
        tempy -= 1;
    }
}
fn is_on_board(xpos: i32, ypos: i32) -> bool {
    if xpos < 8 && xpos >= 0 && ypos < 8 && ypos >=0 {
        return true;
    }
    return false;
}