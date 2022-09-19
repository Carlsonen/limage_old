use limage::*;

fn main() {
    let mut img = Limage::new(501, 501);

    for p in disc((250, 250), 250) {
        img.put_rgb(p, [255; 3]);
    }

    let corners: Vec<(i32, i32)> = vec![(0, 0), (400, 39), (100, 490), (0, 1)];
    let points: Vec<(i32, i32)> = path(&corners).collect();
    for (i, p) in points.iter().enumerate() {
        let hue = i as f32 * 360.0 / points.len() as f32;
        let rgb = hsl_to_rgb([hue, 1.0, 0.5]);
        img.put_rgb(*p, rgb);
    }

    img.save("shit.png").unwrap();
}
