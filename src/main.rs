mod square;
mod pieces;
use minifb::{Key,Window,WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 700;
fn main() {
    let mut buf: Vec<Vec<u32>> = vec![vec![0; WIDTH]; HEIGHT];
    let mut buffer: Vec<u32> = vec![0; WIDTH*HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let c = pieces::ChessPieces::add();
    for (i, a) in buf.iter_mut().enumerate() {
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
    for j in c {
        for (i, a) in j.pic.enumerate_rows() {
            for (mut x, mut y, &rgb) in a {
                x = x + (j.xpos * 80) as u32;
                y = y + (j.ypos * 80) as u32;
                let temp: u32 = ((rgb[0] as u32) << 16) + ((rgb[1] as u32) << 8) + ((rgb[2] as u32));
                if rgb[3]!=0 {
                    buf[y as usize][x as usize] = temp;
                }
            }
        }
    }
    
    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, a) in buf.iter_mut().enumerate() {
            for (j, b) in a.iter_mut().enumerate() {
                buffer[i*640 + j] = *b;
            }
        }
        if let Some((xmouse , ymouse)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if ymouse < 640.0 {
                let mut temp = (xmouse / 80.0).floor() as usize;
                let xi = temp * 80;
                temp = (ymouse / 80.0).floor() as usize;
                let yi = temp * 80;
                for yi in yi..yi+80 {
                    for xi in xi..xi+80 {
                        if buffer[yi*WIDTH + xi] < 0xd1ccbb {
                            buffer[yi*WIDTH + xi] += 0x222222;
                        }    
                    }
                }
            }
        }
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}