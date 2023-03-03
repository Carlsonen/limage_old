use limage::*;
use shiterators::*;

fn main() {
    let mut img = Limage::new(101, 101);
    let points: Vec<(i32, i32)> = shiterators::Disc::new((50, 50), 10).collect();

    for (i, p) in points.iter().enumerate() {
        let hue = 360. * i as f32 / points.len() as f32;
        img.put_rgb(*p, hsl_to_rgb([hue, 1.0, 0.5]));
    }
    img.save("test.png").unwrap();
}

fn bezier(t: f32, points: &Vec<(i32, i32)>) -> (i32, i32) {
    if points.len() < 2 {
        panic!("idiot");
    }
    let mut points: Vec<(f32, f32)> = points.iter().map(|&(x, y)| (x as f32, y as f32)).collect();
    while points.len() > 1 {
        let mut new_points = vec![];
        for i in 0..points.len()-1 {
            new_points.push(lerp(t, points[i], points[i+1]));
        }
        points = new_points.clone();
    }
    let p = points[0];
    (p.0 as i32, p.1 as i32)
}

fn lerp(t: f32, p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    (t * (p2.0-p1.0) + p1.0, t * (p2.1-p1.1) + p1.1)
}
