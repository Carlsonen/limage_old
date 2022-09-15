use limage::*;

fn main() {
    let mut img = Limage::new(100, 100);

    for (x, y) in circle(50, 50, 20) {
        if x >= 0 && y >= 0 {
            img.put_hsl(
                x as u32,
                y as u32,
                [
                    (x + y) as f32 * 360f32 / 40.0,
                    1.0,
                    1.0 - y as f32 / img.height() as f32,
                ],
            );
        }
    }

    img.save("shit.png").unwrap();
}
