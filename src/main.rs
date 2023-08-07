use limage::*;

#[allow(unused)]
fn test1() {
    let mut img = LimageRgb::new(500, 500).with_color([0, 64, 64]);
    let p = 1.0;
    let q = -1.0;
    let vertex_table = vec![
        (q, q, q),
        (q, q, p),
        (q, p, p),
        (q, p, q),
        (p, q, q),
        (p, q, p),
        (p, p, p),
        (p, p, q),
    ];
    let edge_table = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    for p in WireFrame::from_3d(&vertex_table, 2.5, &edge_table, 500) {
        img.put_rgb(p, [255; 3]);
    }
    img.save("test.png").unwrap();
}

fn test2() {
    let img = from_google("lava", PexelMode::Landscape).to_limage_rgba();
    img.save("google.png").unwrap();
}
fn main() {
    test2();
}
