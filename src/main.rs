use limage::{prelude::*, shiterators::Text};

fn main() {
    let mut img = Limage::new(600, 100).with_color([10; 3]);

    for p in Text::new((30, 20), "Qwerasd oscar91 =)".to_string(), 5) {
        let hue = p.0 - 30;
        img.put_rgb(p, hsl_to_rgb([hue as f32, 1.0, 0.5]));
    }

    img.save("text.png").unwrap();
}
