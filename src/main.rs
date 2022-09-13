use limage::{patterns::Grid, Limage};

fn main() {
    let img = Limage::new(360, 360)
        .with_color([255; 3])
        .with_pattern(Grid::new(3, 20, 20));

    img.save("shit.png").unwrap();
}
