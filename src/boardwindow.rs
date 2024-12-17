use minifb::{Key, Window, WindowOptions};
use crate::board::Board;

pub struct WindowFrame {
    pub window_frame: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    pub offset: usize
}

impl WindowFrame {
    pub fn setup_window() -> WindowFrame {
        let mut new_window = Window::new(
            "Test - ESC to exit",
            640,
            760,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        new_window.set_target_fps(60);
        let new_buffer = vec![0x97572B; 640*760];
        WindowFrame {window_frame: new_window, buffer: new_buffer, width: 640, height: 760, offset: 38400}
    }

    fn update_window(&mut self) {
        self.window_frame.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    pub fn is_open(&mut self) -> bool {
        if self.window_frame.is_open() && !self.window_frame.is_key_down(Key::Escape) {
            return true;
        }
        return false;
    }

    pub fn board_to_window(&mut self, board: &Board) {
        for (board_x,column) in board.board.iter().enumerate() {
            for (board_y,square) in column.iter().enumerate() {
                let mut x_square: usize = 0;
                let mut y_square: usize = 0;
                while x_square < square.size {
                    while y_square < square.size {
                        //board position * square size gets the start of that square
                        //we multiply ypositions by 640 because the start of the second lines position in memory is after 640 pixels on first row 
                        self.buffer[(board_y * square.size + y_square) * self.width + (board_x * 80 + x_square) + self.offset] = square.square[x_square][y_square];
                        y_square += 1;
                    }
                    y_square = 0;
                    x_square +=1;
                }
            }
        }
        self.update_window();
    }
}
