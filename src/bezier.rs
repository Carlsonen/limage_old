use crate::{line, Limage};

type Coord = (f32, f32);

#[derive(Clone, Copy, PartialEq, Default)]
pub struct CubicBezier {
    pub endpoints: (Coord, Coord),
    pub controls: (Coord, Coord),
}

impl CubicBezier {
    pub fn new(p1: (f32, f32), p2: (f32, f32)) -> Self {
        Self {
            endpoints: (p1, p2),
            controls: (p1, p2),
        }
    }

    pub fn value(&self, t: f32) -> (f32, f32) {
        let calc = |u1, u2, u3, u4| {
            (1. - t).powi(3) * u1
                + 3. * t * (1. - t).powi(2) * u2
                + 3. * t.powi(2) * (1. - t) * u3
                + t.powi(3) * u4
        };

        (
            calc(
                self.endpoints.0 .0,
                self.controls.0 .0,
                self.controls.1 .0,
                self.endpoints.1 .0,
            ),
            calc(
                self.endpoints.0 .1,
                self.controls.0 .1,
                self.controls.1 .1,
                self.endpoints.1 .1,
            ),
        )
    }

    pub fn draw(&self, img: &mut Limage, color: [u8; 3]) {
        for i in 1..=100 {
            let t_prev = (i - 1) as f32 / 100.;
            let t = i as f32 / 100.;

            let last = self.value(t_prev);
            let now = self.value(t);

            let last = (
                (last.0 * img.width() as f32) as i32,
                (last.1 * img.height() as f32) as i32,
            );
            let now = (
                (now.0 * img.width() as f32) as i32,
                (now.1 * img.height() as f32) as i32,
            );

            let line = line(last, now);
            for p in line {
                img.put_rgb(p, color);
            }
        }
    }
}
