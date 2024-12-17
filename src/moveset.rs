use crate::{moveset, piece::{self, ChessPiece, Colour, Piece}};

pub fn kingmove(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    for xm in [-1,0,1].iter() {
        for ym in [-1,0,1].iter() {
            if *xm == 0 && *ym == 0 {continue;}
            let xmove: usize = (x as i32 + *xm) as usize;
            let ymove: usize = (y as i32 + *ym) as usize;
            if !on_board(&xmove, &ymove) {continue;}
            if let None = pieces[ymove][xmove] {
                if !causes_check((x,y), (xmove,ymove), pieces, turn) {
                    result.push((xmove,ymove));
                }
            }
        }
    }
    result.append(&mut castle(x, y, pieces, turn));
    return result;
}
pub fn queenmove(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    result.append(&mut diagonalmove(x.clone(), y.clone(), pieces, turn));
    result.append(&mut straightmove(x, y, pieces, turn));
    return result;
}

pub fn pawnmove(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour, last_move: &((i32,i32), (i32,i32))) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    let mut pawn = pieces[y][x].clone();
    match pawn.as_ref().unwrap().colour {
        Colour::BLACK => {
            if !causes_check((x,y), (x,y+1), pieces, turn) {
                result.push((x, y + 1));
            }
            if !causes_check((x,y), (x,y+2), pieces, turn) {
                if !pawn.as_ref().unwrap().moved {result.push((x, y + 2));}
            }
        }
        Colour::WHITE => {
            if !causes_check((x,y), (x,(y as i32 -1) as usize), pieces, turn) {
                result.push((x, (y as i32 - 1) as usize));
            }
            if !causes_check((x,y), (x, (y as i32 - 2) as usize), pieces, turn) {
                if !pawn.as_ref().unwrap().moved {result.push((x, (y as i32 - 2) as usize));}
            }
        }
    }
    result.append(&mut enpassant(x, y, pieces, turn, last_move));
    return result;
}

pub fn knightmove(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    for xm in -2..3 as i32 {
        for ym in -2..3 as i32 {
            if xm == 0 || ym == 0 || xm.abs() + ym.abs() > 3 {continue;}
            let xmove = (x as i32 + xm) as usize;
            let ymove = (y as i32 + ym) as usize;
            if on_board(&xmove, &ymove) && !causes_check((x,y), (xmove,ymove), pieces, turn) {
                result.push((xmove,ymove));
            }
        }
    }
    return result;
}

pub fn straightmove(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    let mut xm: usize = x+1;
    while on_board(&xm, &y) {
        if !causes_check((x,y), (xm, y), pieces, turn) {
            if let Some(piece) = pieces[y][xm].as_ref() {
                if piece.colour != *turn {
                    result.push((xm,y));
                }
                break;
            }
        }
        xm = xm + 1;
    }
    xm = (x as i32- 1) as usize;
    while on_board(&xm, &y) {
        if !causes_check((x,y), (xm, y), pieces, turn) {
            if let Some(piece) = pieces[y][xm].as_ref() {
                if piece.colour != *turn {
                    result.push((xm,y));
                }
                break;
            }
        }
        xm = (xm as i32 - 1) as usize;
    }
    let mut ym = y + 1;
    while on_board(&x, &ym) {
        if !causes_check((x,y), (x, ym), pieces, turn) {
            if let Some(piece) = pieces[ym][x].as_ref() {
                if piece.colour != *turn {
                    result.push((x,ym));
                }    
                break;
            }
        }
        ym = ym + 1;
    }
    ym = (y as i32 - 1) as usize;
    while on_board(&x, &ym) {
        if !causes_check((x,y), (x, ym), pieces, turn) {
            if let Some(piece) = pieces[ym][x].as_ref() {
                if piece.colour != *turn {
                    result.push((x,ym));
                }    
                break;
            }
        }
        ym = ym - 1;
    }
    return result;
}

pub fn diagonalmove(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();

    let mut xm: usize = x+1;
    let mut ym: usize = y+1;
    while on_board(&xm, &ym) {
        if !causes_check((x,y), (xm, ym), pieces, turn) {
            if let Some(piece) = pieces[ym][xm].as_ref() {
                if piece.colour != *turn {
                    result.push((xm,ym));
                }
                break;
            }
        }
        xm += 1;
        ym += 1;
    }
    xm = x + 1;
    ym = (y as i32 - 1) as usize;
    while on_board(&xm, &ym) {
        if !causes_check((x,y), (xm, ym), pieces, turn) {
            if let Some(piece) = pieces[ym][xm].as_ref() {
                if piece.colour != *turn {
                    result.push((xm,ym));
                }
                break;
            }
        }
        xm = xm + 1;
        ym = (ym as i32 - 1) as usize;
    }
    ym = y + 1;
    xm = (x as i32 - 1) as usize;
    while on_board(&xm, &ym) {
        if !causes_check((x,y), (xm, ym), pieces, turn) {
            if let Some(piece) = pieces[ym][xm].as_ref() {
                if piece.colour != *turn {
                    result.push((xm,ym));
                }    
                break;
            }
        }
        ym = ym + 1;
        xm = (xm as i32 - 1) as usize;
    }
    ym = (y as i32 - 1) as usize;
    xm = (x as i32 - 1) as usize;
    while on_board(&xm, &ym) {
        if !causes_check((x,y), (xm, ym), pieces, turn) {
            if let Some(piece) = pieces[ym][xm].as_ref() {
                if piece.colour != *turn {
                    result.push((xm,ym));
                }    
                break;
            }
        }
        ym = (ym as i32 - 1) as usize;
        xm = (xm as i32 - 1) as usize;
    }
    
    return result;
}

pub fn enpassant(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour, last_move: &((i32,i32), (i32,i32))) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    let last_move_to_x = last_move.1.0 as usize;
    let last_move_to_y = last_move.1.1 as usize;
    let last_move_from_y = last_move.0.1 as usize;
    if !on_board(&last_move_to_x, &last_move_to_y) || pieces[last_move_to_y][last_move_to_x].is_none() {return result;}
    if pieces[last_move_to_y][last_move_to_x].as_ref().unwrap().role == Piece::Pawn  && (last_move_to_y as i32 - last_move_from_y as i32).abs() > 1{
        let moved_pawn = pieces[last_move_to_y][last_move_to_x].as_ref().unwrap();
        let xmoves = [last_move_to_x -1, last_move_to_x + 1];
        let ymoves: [usize;2];
        if moved_pawn.colour == Colour::WHITE {
            ymoves = [last_move_to_y, last_move_to_y + 1];
        }else {
            ymoves = [last_move_to_y, last_move_to_y - 1];
        }
        for ym in ymoves {
            for xm in xmoves {
                if on_board(&xm, &ym) && !causes_check((x,y), (xm,ym), pieces, turn) {
                    result.push((xm,ym));
                }
            }
        }
    }

    return result;
}

pub fn castle(x: usize, y: usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> Vec<(usize,usize)>{
    let mut result: Vec<(usize,usize)> = Vec::new();
    let mut long: bool = true;
    let mut short: bool = false;
    if let Some(king) = pieces[y][x].clone()  {
        let ym = match king.colour {
            Colour::WHITE=> 0,
            Colour::BLACK=> 7
        };
        if !king.moved {
            if causes_check((x,y), (x,y), pieces, turn) {long = false; short = false;}
            if causes_check((x,y), (x+1, y), pieces, turn) || causes_check((x,y), (x+2, y), pieces, turn) {long = false;}
            if causes_check((x,y), (x-1, y), pieces, turn) || causes_check((x,y), (x-2, y), pieces, turn) {short = false;}
            if short {result.push((x-2,y));}
            if long {result.push((x+2,y));}
        }
    }
    return result;
}
fn causes_check(start_pos: (usize,usize), end_pos: (usize,usize) , pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> bool{
    let mut copy_pieces = pieces.clone();
    let copy_piece = copy_pieces[start_pos.1 as usize][start_pos.0 as usize].clone();
    if !on_board(&end_pos.0, &end_pos.1) {return true;}

    copy_pieces[start_pos.1][start_pos.0] = None;
    copy_pieces[end_pos.1][end_pos.0] = copy_piece.clone();

    return is_check(&copy_pieces, turn);
}

pub fn is_check(pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> bool {
    let mut kingpos: (usize, usize) = (9, 9);
    for y in 0..8 {
        for x in 0..8 {
            if pieces[y][x].is_none() {continue;}
            if pieces[y][x].as_ref().unwrap().role == Piece::King && pieces[y][x].as_ref().unwrap().colour == *turn {
                kingpos = (x,y);
            }
        }
    }
    for x in [-1 as i32,1].iter() {
        if *turn == Colour::WHITE {
            if !on_board(&((kingpos.0 as i32 + *x) as usize), &((kingpos.1 as i32 - 1)as usize)) {continue;}
            if let Some(piece) = pieces[kingpos.1 - 1][(kingpos.0 as i32 + *x) as usize].as_ref() {
                if piece.colour != *turn {
                    match piece.role {
                        Piece::Pawn => return true,
                        _ => continue
                    };
                }
            }
        }else {
            if !on_board(&((kingpos.0 as i32 + *x) as usize), &((kingpos.1 as i32 + 1)as usize)) {continue;}
            if let Some(piece) = pieces[kingpos.1 + 1][(kingpos.0 as i32 + *x) as usize].as_ref() {
                if piece.colour != *turn {
                    match piece.role {
                        Piece::Pawn => return true,
                        _ => continue
                    };
                }
            }
        }
    }
    if check_diagonals(1, 1, &kingpos, pieces, &turn) {return true;}
    if check_diagonals(1, -1, &kingpos, pieces, &turn) {return true;}
    if check_diagonals(-1, 1, &kingpos, pieces, &turn) {return true;}
    if check_diagonals(-1, -1, &kingpos, pieces, &turn) {return true;}
    if check_horse(&kingpos, pieces, &turn) {return true;}
    if check_straights(&kingpos, pieces, turn) {return true;}
    return false;
}
fn check_straights(kingpos: &(usize, usize), pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> bool {
    let mut x = kingpos.0;
    let mut y = kingpos.1;
    x = (x as i32 + 1) as usize;
    while on_board(&x, &y) {
        if let Some(piece) = pieces[y][x].as_ref() {
            if piece.colour != *turn {
                match piece.role {
                    Piece::Queen => return true,
                    Piece::Bishop => return true,
                    _ =>  break
                };
            }
        }
        x = (x as i32 + 1) as usize;
    }
    x = kingpos.0;
    y = kingpos.1;
    x = (x as i32 - 1) as usize;
    while on_board(&x, &y) {
        if let Some(piece) = pieces[y][x].as_ref() {
            if piece.colour != *turn {
                match piece.role {
                    Piece::Queen => return true,
                    Piece::Bishop => return true,
                    _ => break
                };
            }
        }
        x = (x as i32 - 1) as usize;
    }
    x = kingpos.0;
    y = kingpos.1;
    y = (y as i32 + 1) as usize;
    while on_board(&x, &y) {
        if let Some(piece) = pieces[y][x].as_ref() {
            if piece.colour != *turn {
                match piece.role {
                    Piece::Queen => return true,
                    Piece::Bishop => return true,
                    _ => break
                };
            }
        }
        y = (y as i32 + 1) as usize;
    }
    x = kingpos.0;
    y = kingpos.1;
    y = (y as i32 - 1) as usize;
    while on_board(&x, &y) {
        if let Some(piece) = pieces[y][x].as_ref() {
            if piece.colour != *turn {
                match piece.role {
                    Piece::Queen => return true,
                    Piece::Bishop => return true,
                    _ => break
                };
            }
        }
        y = (y as i32 - 1) as usize;
    }
    return false;
}

fn check_diagonals(diffx: i32, diffy: i32, kingpos: &(usize, usize), pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> bool {
    let mut x = kingpos.0;
    let mut y = kingpos.1;
    x = (x as i32 + diffx) as usize;
    y= (y as i32 + diffy) as usize;

    while on_board(&x, &y) {
        if let Some(piece) = pieces[y][x].as_ref() {
            if piece.colour != *turn {
                match piece.role {
                    Piece::Queen => return true,
                    Piece::Bishop => return true,
                    _ => break
                };
            }
        }
        x = (x as i32 + diffx) as usize;
        y= (y as i32 + diffy) as usize;
    }
    return false;
}

fn check_horse(kingpos: &(usize, usize), pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour) -> bool {
    for xdiff in -2..3 as i32 {
        if xdiff == 0 {continue;}
        for ydiff in -2..3 as i32 {
            if xdiff.abs() + ydiff.abs() == 4 {continue;}
            let x: usize = (kingpos.0 as i32 + xdiff) as usize;
            let y: usize = (kingpos.1 as i32 + ydiff) as usize;
            if !on_board(&x, &y) {continue;}
            if let Some(piece) = pieces[y][x].as_ref() {
                if piece.colour != *turn {
                    match piece.role {
                        Piece::Knight => return true,
                        _ => continue
                    };
                }
            }
        }
    }
    return false;
}

fn on_board(x: &usize, y: &usize) -> bool {
    if *x < 8 && *y < 8 {
        return true;
    }
    return false;
}