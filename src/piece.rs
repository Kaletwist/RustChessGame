use image::{imageops, open, DynamicImage, ImageBuffer, Rgba};
use strum_macros::EnumIter;
use strum::IntoEnumIterator; 
use crate::moveset;

#[derive(PartialEq,EnumIter, Clone, Copy, Debug)]
pub enum Colour{
    WHITE = 1,
    BLACK = 0
}
#[derive(PartialEq,EnumIter,Clone, Copy,Debug)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Pawn = 5
}
#[derive(Clone, Debug)]
pub struct ChessPiece {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub moveset: Vec<(usize, usize)>,
    pub role: Piece,
    pub colour: Colour,
    pub moved: bool
}

impl ChessPiece{
    pub fn new(r: &Piece, c: &Colour, pieces_img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ChessPiece {
        ChessPiece{
            image: pieces_img.clone(),
            moveset: Vec::new(),
            role: *r,
            colour: *c,
            moved: false
        }
    }
    fn update_view(&mut self, x: &usize, y: &usize, pieces: &Vec<Vec<Option<ChessPiece>>>, turn: &Colour, last_move: &((i32, i32), (i32, i32))) {
        let moves: Vec<(usize, usize)> = match self.role {
            Piece::King => moveset::kingmove(*x,*y, pieces, turn),
            Piece::Queen => moveset::queenmove(*x,*y, pieces, turn),
            Piece::Bishop => moveset::diagonalmove(*x,*y, pieces, turn),
            Piece::Knight => moveset::knightmove(*x,*y, pieces, turn),
            Piece::Rook => moveset::straightmove(*x,*y, pieces, turn),
            Piece::Pawn => moveset::pawnmove(*x,*y, pieces, turn, last_move),
        };
        self.moveset = moves.clone();
        println!("{}", moves.len());
    }
    pub fn no_moves(&self) -> bool{
        if self.moveset.is_empty() {return true;}
        return false;
    }
    pub fn get_moves(&mut self) -> Vec<(usize, usize)> {
        return self.moveset.clone();
    }
}
pub fn update_views(pieces: &mut Vec<Vec<Option<ChessPiece>>>, turn: &Colour, last_move: &((i32,i32), (i32,i32))) {
    let temp_pieces = pieces.clone();
    for y in 0..8 {
        for x in 0..8 {
            if pieces[y][x].is_none() {continue;}
            pieces[y][x].as_mut().unwrap().update_view(&x, &y, &temp_pieces, turn, &last_move);
        }
    }
}

pub fn init_pieces() -> Vec<Vec<Option<ChessPiece>>> {
    let mut result:  Vec<Vec<Option<ChessPiece>>> = vec![vec![None;8];8];
    let pieces_img: DynamicImage = open("C:/Users/junio/OneDrive/Desktop/ChessBoard/Chessboard2/ChessBoard2/Chess_Pieces_Sprite.svg.png").unwrap();
    let buf_pieces: ImageBuffer<Rgba<u8>, Vec<u8>> = pieces_img.into_rgba8();

    for colour in Colour::iter() {
        let mut ypos: usize;
        if colour == Colour::WHITE {
            ypos = 7;
        }
        else {ypos = 0}
        for role in Piece::iter() {
            let xpositions: Vec<usize> = match role {
                Piece::King => vec![3],
                Piece::Queen => vec![4],
                Piece::Bishop => vec![2,5],
                Piece::Knight => vec![1,6],
                Piece::Rook => vec![0,7],
                Piece::Pawn => vec![0,1,2,3,4,5,6,7]
            };
            if role == Piece::Pawn && colour == Colour::WHITE {ypos -= 1;}
            else if role == Piece::Pawn {ypos += 1;}
             
            for xpos in  xpositions {
                result[ypos][xpos] = Some(ChessPiece::new(&role, &colour, &get_piece_img(&role, &colour, &buf_pieces)));
            }
        }
    }
    return result;
}

fn get_piece_img(role: &Piece, colour: &Colour, full_image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let piece_height = full_image.height()/2;
    let piece_width = full_image.width()/6;
    let mut piece_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(piece_width, piece_height);
    let y_start = match *colour {   
        Colour::BLACK=> piece_height,
        Colour::WHITE=> 0
    };
    let x_start: u32 = match *role {
        Piece::King => 0,
        Piece::Queen => piece_width,
        Piece::Bishop => piece_width * 2,
        Piece::Knight => piece_width * 3,
        Piece::Rook => piece_width * 4,
        Piece::Pawn => piece_width * 5
    };
    
    for x in x_start..x_start+piece_width {
        for y in y_start..y_start+piece_height {
            piece_img.put_pixel(x- x_start, y - y_start, *full_image.get_pixel(x, y));
        }
    }
    let mut resize = DynamicImage::ImageRgba8(piece_img);
    resize = resize.resize_exact(80, 80, imageops::FilterType::Gaussian);
    piece_img = resize.to_rgba8();
    return piece_img;
}
