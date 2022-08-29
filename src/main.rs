use limage::Limage;

fn main() {
    let img = Limage::new(360, 360);

    img.save("shit.png").unwrap();
}
