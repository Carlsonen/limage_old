use limage::*;
use shiterators::{Disc, Text};

fn main() {
    let mut img = LimageRgb::new(500, 500).with_color([0, 64, 64]);
    let p = 1.0;
    let q = -1.0;
    let vertex_table = vec![(q,q,q),(q,q,p),(q,p,p),(q,p,q),(p,q,q),(p,q,p),(p,p,p),(p,p,q)];
    let edge_table = vec![(0,1),(1,2),(2,3),(3,0),
                          (4,5),(5,6),(6,7),(7,4),
                          (0,4),(1,5),(2,6),(3,7)];
    for p in WireFrame::from_3d(&vertex_table, 2.5, &edge_table, 500) {
        img.put_rgb(p, [255; 3]);
    }
    println!("{:?}", sizeof_text("helloworld", 40.0, "TumsBasic.ttf"));
    img.write_text((0,0), [255; 3], "helloworld", 40.0, "TumsBasic.ttf");
    img.save("test.png").unwrap();
}

fn bezier(t: f32, points: &Vec<(i32, i32)>) -> (i32, i32) {
    if points.len() < 2 {
        panic!("idiot");
    }
    let mut points: Vec<(f32, f32)> = points.iter().map(|&(x, y)| (x as f32, y as f32)).collect();
    while points.len() > 1 {
        let mut new_points = vec![];
        for i in 0..points.len() - 1 {
            new_points.push(lerp(t, points[i], points[i + 1]));
        }
        points = new_points.clone();
    }
    let p = points[0];
    (p.0 as i32, p.1 as i32)
}

fn lerp(t: f32, p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    (t * (p2.0 - p1.0) + p1.0, t * (p2.1 - p1.1) + p1.1)
}
