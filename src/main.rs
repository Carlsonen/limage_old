use limage::prelude::*;

fn main() {
    let mut img = Limage::new(501, 501).with_color(RED);

    for p in Circle::new((250, 250), 100) {
        img.put_rgb(p, GREEN);
    }

    for o in Line::new((10, 10), (490, 490)).step_by(7) {
        for p in Circle::new(o, 10) {
            img.put_rgb(p, GREEN);
        }
    }

    img.save("shit.png").unwrap();
}
