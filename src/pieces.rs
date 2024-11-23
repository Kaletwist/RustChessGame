use image::{imageops, open, DynamicImage, GenericImage, ImageBuffer, ImageReader, Pixel, RgbImage, Rgba};
use std::path::Path;
pub struct ChessPieces {
    pub xpos: i32,
    pub ypos: i32,
    pub piece: String,
    pub pic: ImageBuffer<Rgba<u8>, Vec<u8>>
}

impl ChessPieces {
    pub fn add() -> Vec<ChessPieces> {
        let mut result: Vec<ChessPieces> = Vec::new();
        let path = Path::new("C:/Users/junio/OneDrive/Desktop/ChessBoard/ChessBoard/Chess_Pieces_Sprite.svg.png");
        let all = open(path).unwrap();
        let bufall = all.into_rgba8();
        for i in 0..12 {
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
            img = t.to_rgba8();
            match i {
                0 => result.push(ChessPieces::new("White King", &img, 3, 7)),
                1 => result.push(ChessPieces::new("White Queen", &img, 4, 7)),
                2 => {
                    result.push(ChessPieces::new("White Bishop", &img, 2, 7));
                    result.push(ChessPieces::new("White Bishop", &img, 5, 7));
                    },
                3 => {
                    result.push(ChessPieces::new("White Knight", &img, 1, 7));
                    result.push(ChessPieces::new("White Knight", &img, 6, 7));
                }
                4 => {
                    result.push(ChessPieces::new("White Rook", &img, 0, 7));
                    result.push(ChessPieces::new("White Rook", &img, 7, 7));
                },
                5 => {
                    for pawn_num in 0..8 {
                        result.push(ChessPieces::new("White Pawn", &img, pawn_num, 6));
                    }
                },
                6 => result.push(ChessPieces::new("Black King", &img, 3, 0)),
                7 => result.push(ChessPieces::new("Black Queen", &img, 4, 0)),
                8 => {
                    result.push(ChessPieces::new("Black Bishop", &img, 2, 0));
                    result.push(ChessPieces::new("Black Bishop", &img, 5, 0));
                    },
                9 => {
                    result.push(ChessPieces::new("Black Knight", &img, 1, 0));
                    result.push(ChessPieces::new("Black Knight", &img, 6, 0));
                }
                10 => {
                    result.push(ChessPieces::new("Black Rook", &img, 0, 0));
                    result.push(ChessPieces::new("Black Rook", &img, 7, 0));
                },
                11 => {
                    for pawn_num in 0..8 {
                        result.push(ChessPieces::new("Black Pawn", &img, pawn_num, 1));
                    }
                },
                _ => panic!("Out of bounds")
            }
        }
        return result;
    }
    fn new(s: &str , im: &ImageBuffer<Rgba<u8>, Vec<u8>>, x: i32, y: i32) -> ChessPieces {
        ChessPieces {
            xpos: x,
            ypos: y,
            piece: s.to_string(),
            pic: im.clone()
        }
    }
}