struct Point {
    x: d32,
    y: d32,
    z: d32,
    w: i32,
}
impl Point {
    fn new(xx: d32, yy: d32, zz: d32, ww: i32) -> Point {
        Point{
            x: xx,
            y: yy,
            z: zz,
            w: ww
        }
    }
}