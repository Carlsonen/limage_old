use crate::shiterators::*;

mod limage;

pub use limage::Limage;

pub fn rectangle(p1: (i32, i32), p2: (i32, i32)) -> Rectangle {
    let (x, y) = p1;
    let (w, h) = p2;
    Rectangle::new(x, y, w, h)
}

pub fn circle(origin: (i32, i32), radius: u32) -> Circle {
    let (x, y) = origin;
    Circle::new((x, y), radius)
}

pub fn disc(origin: (i32, i32), radius: u32) -> Disc {
    let (x, y) = origin;
    Disc::new(x, y, radius as i32)
}

pub fn line(p1: (i32, i32), p2: (i32, i32)) -> Line {
    Line::new(p1, p2)
}

pub fn path(points: &Vec<(i32, i32)>) -> Path {
    Path::new(points)
}
