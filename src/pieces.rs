use image::{imageops, open, DynamicImage, ImageBuffer, Rgba};
use std::path::Path;
use crate::moveset;
#[derive(Clone)]
pub struct ChessPieces {
    pub xpos: i32,
    pub ypos: i32,
    pub role: String,
    pub colour: String,
    pub views: Vec<(i32, i32)>,
    pub pic: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub moved: bool
}

impl ChessPieces {
    pub fn init() -> Vec<ChessPieces> {
        let mut result: Vec<ChessPieces> = Vec::new();
        let path = Path::new("Chess_Pieces_Sprite.svg.png");
        let all = open(path).unwrap();
        let bufall = all.into_rgba8();
        for i in 0..12 {
            let img = get_piece_img(&bufall, i);
            match i {
                0 => result.push(ChessPieces::new("White", "King", &img, 3, 7)),
                1 => result.push(ChessPieces::new("White", "Queen", &img, 4, 7)),
                2 => {
                    result.push(ChessPieces::new("White", "Bishop", &img, 2, 7));
                    result.push(ChessPieces::new("White", "Bishop", &img, 5, 7));
                    },
                3 => {
                    result.push(ChessPieces::new("White", "Knight", &img, 1, 7));
                    result.push(ChessPieces::new("White", "Knight", &img, 6, 7));
                }
                4 => {
                    result.push(ChessPieces::new("White", "Rook", &img, 0, 7));
                    result.push(ChessPieces::new("White", "Rook", &img, 7, 7));
                },
                5 => {
                    for pawn_num in 0..8 {
                        result.push(ChessPieces::new("White", "Pawn", &img, pawn_num, 6));
                    }
                },
                6 => result.push(ChessPieces::new("Black", "King", &img, 3, 0)),
                7 => result.push(ChessPieces::new("Black", "Queen", &img, 4, 0)),
                8 => {
                    result.push(ChessPieces::new("Black", "Bishop", &img, 2, 0));
                    result.push(ChessPieces::new("Black", "Bishop", &img, 5, 0));
                    },
                9 => {
                    result.push(ChessPieces::new("Black", "Knight", &img, 1, 0));
                    result.push(ChessPieces::new("Black", "Knight", &img, 6, 0));
                }
                10 => {
                    result.push(ChessPieces::new("Black", "Rook", &img, 0, 0));
                    result.push(ChessPieces::new("Black", "Rook", &img, 7, 0));
                },
                11 => {
                    println!();
                    for pawn_num in 0..8 {
                        result.push(ChessPieces::new("Black", "Pawn", &img, pawn_num, 1));
                    }
                },
                _ => panic!("Out of bounds")
            }
        }
        return result;
    }
    fn new(c: &str, s: &str , im: &ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32) -> ChessPieces {
        ChessPieces {
            xpos: x,
            ypos: y,
            role: s.to_string(),
            colour: c.to_string(),
            views: Vec::new(),
            pic: im.clone(),
            moved: false
        }
    }
    pub fn set_views(&mut self, square_contains: &Vec<Vec<i8>>) {
        let role = self.role.as_str();
        self.views = match role {
            "King" => moveset::king(&self, square_contains),
            "Rook" => moveset::rook(&self, square_contains),
            "Queen" => moveset::queen(&self, square_contains),
            "Bishop" => moveset::bishop(&self, square_contains),
            "Knight" => moveset::knight(&self, square_contains),
            "Pawn" => moveset::pawn(&self, square_contains),
            _ => panic!("Not a Piece")
        }
    }
}

pub fn get_piece_img(bufall: &ImageBuffer<Rgba<u8>, Vec<u8>>, i: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(bufall.width()/6, bufall.height()/2);
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut y_bufall: u32 = 0;
    if i > 5 {
        y_bufall = bufall.height()/2;
    }
    let x_bufall: u32 = (bufall.width() / 6) * (i % 6);    
    while y < bufall.height()/2 {
        img.put_pixel(x, y, *bufall.get_pixel(x + x_bufall, y + y_bufall));
        x+= 1;
        if x >= (bufall.width() / 6) {
            x = 0;
            y += 1;
        }
    }
    let mut t: DynamicImage = DynamicImage::ImageRgba8(img);
    t = t.resize_exact(80, 80, imageops::FilterType::Gaussian);
    return t.to_rgba8();
}
