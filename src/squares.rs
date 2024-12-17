use crate::piece::ChessPiece;
pub struct Square {
    position: (u32,u32),
    pub size: usize,
    pub piece: Option<ChessPiece>,
    pub square: [[u32; 80]; 80],
    base_colour: u32
}

impl Square {
    pub fn new(xloc: u32, yloc: u32) -> Square {
        let this_colour: u32;
        if (xloc % 2 == 1 && yloc % 2 == 1) || (yloc % 2 == 0 && xloc % 2 == 0) {
            this_colour = 0xd1ccba;
        }else {
            this_colour = 0x006600;
        }
        Square {
            position: (xloc,yloc),
            size: 80,
            piece: None,
            square: [[this_colour;80];80],
            base_colour: this_colour
        }
    }
    pub fn highlight(&mut self) {
        for y in self.square.iter_mut() {
            for x in y.iter_mut() {
                if *x == self.base_colour {*x += 0x222222;}
            }
        }
    }
    pub fn undo_highlight(&mut self) {
        for y in self.square.iter_mut() {
            for x in y.iter_mut() {
                if *x == (self.base_colour + 0x222222) {*x -= 0x222222;}
            }
        }
    }
    pub fn place_piece(&mut self) {
        if let Some(piece) = &self.piece {
            for (x, row) in self.square.iter_mut().enumerate() {
                for (y, column) in row.iter_mut().enumerate() {
                    let rgb = piece.image[(x as u32,y as u32)]; 
                    let temp: u32 = ((rgb[0] as u32) << 16) + ((rgb[1] as u32) << 8) + ((rgb[2] as u32));
                    if rgb[3] != 0 {
                        *column = temp;
                    }
                }
            }
        }
    }
    pub fn pickup_piece(&mut self) {
        self.square = [[self.base_colour;80];80];
    }
}