use minifb::{Key,Window,WindowOptions};
const WIDTH: usize = 640;
const HEIGHT: usize = 700;
fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    for (i, a) in buffer.iter_mut().enumerate() {
        if (i % 160 < 80   &&  i % (160*640) < (80*640) && i < (640*640)) || (i % 160 >= 80 &&  i % (160*640) >= (80*640) && i < (640*640))   {
            *a = 0xDCD7C6;
        }else if i > 640*640{
            *a = 0x97572B;
        }else {
            *a = 0x009900;
        }
    }
    
    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}