use limage::*;

fn main() {
    let mut img = 
        Limage::new(500, 500)
        .with_color(BEIGE);

    for origin in line((0,0), (250,250)).step_by(10) {
        for p in circle(origin, 5) {
            img.put_rgb(p, FOREST_GREEN);
        }
    }

    for p2 in circle((400,250), 40) {
        for (i, p) in line((300,400), p2).enumerate() {
            img.put_rgb(p, [i as u8, 0, 0]);
        }
    }

    img.save("shit.png").unwrap();
}
