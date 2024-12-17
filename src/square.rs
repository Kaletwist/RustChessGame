pub struct Squares{
    row: i32,
    column: i32,
    piece: i32
}
impl Squares {
    pub fn new(r: i32, c: i32, p: i32)-> Squares {
        Squares{        
            row: r,
            column: c,
            piece: p
        }
    }
}