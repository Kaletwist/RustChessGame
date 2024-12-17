use image::ImageBuffer;
use image::open;
use image::Rgb;
use image::Pixel;
struct Pieces {
    king: ImageBuffer<Rgb<f32>, Vec<f32>>,
    queen: ImageBuffer<Rgb<f32>, Vec<f32>>,
    rook: ImageBuffer<Rgb<f32>, Vec<f32>>,
    knight: ImageBuffer<Rgb<f32>, Vec<f32>>,
    bishop: ImageBuffer<Rgb<f32>, Vec<f32>>,
    pawn: ImageBuffer<Rgb<f32>, Vec<f32>>
}
impl Pieces {
    fn new_white() -> Pieces {
        let img = image_in();
        let imgtemp: ImageBuffer<Rgb<f32>, Vec<f32>>;
        for y in 27..107 as u32 {
            for x in 35..115 as u32 {
                imgtemp[y][x] = img[y][x];
            }
        }
        Pieces {
            king: imgtemp,
            queen: imgtemp,
            rook: imgtemp,
            knight: imgtemp,
            bishop: imgtemp,
            pawn: imgtemp,
        }    
    }
}

fn image_in<Container, P: image::Pixel>() -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    return open("Chess_Pieces_Sprite.svg.png").unwrap().into_rgb32f();
}