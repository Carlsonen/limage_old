use std::path::Path;

use image::{ImageBuffer, ImageResult, RgbImage};
use patterns::Pattern;

pub mod patterns;

pub struct Limage {
    pub imgbuff: RgbImage,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coords {
    Whole(i64, i64),
    Fractional(f32, f32),
}

impl From<(i64, i64)> for Coords {
    fn from(coords: (i64, i64)) -> Self {
        Self::Whole(coords.0, coords.1)
    }
}

impl From<(f32, f32)> for Coords {
    fn from(coords: (f32, f32)) -> Self {
        Self::Fractional(coords.0, coords.1)
    }
}

impl From<(u32, u32)> for Coords {
    fn from(coords: (u32, u32)) -> Self {
        Self::Whole(coords.0 as i64, coords.1 as i64)
    }
}

impl Coords {
    pub fn convert(self, width: u32, height: u32) -> Result<(u32, u32), LimageError> {
        match self {
            Self::Whole(x, y) if x >= 0 && (x as u32) < width && y >= 0 && (y as u32) < height => {
                Ok((x as u32, y as u32))
            }
            Self::Fractional(x, y) if (0. ..=1.).contains(&x) && (0. ..=1.).contains(&y) => Ok((
                (x * (width as f32 - 1.)) as u32,
                (y * (height as f32 - 1.)) as u32,
            )),
            _ => Err(LimageError::OutOfBounds),
        }
    }

    pub fn convert_unchecked(self, width: u32, height: u32) -> (i64, i64) {
        match self {
            Self::Whole(x, y) => (x, y),
            Self::Fractional(x, y) => (
                (x * (width as f32 - 1.)) as i64,
                (y * (height as f32 - 1.)) as i64,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LimageError {
    OutOfBounds,
}

// make - save
impl Limage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            imgbuff: ImageBuffer::new(width, height),
        }
    }

    pub fn with_color(mut self, color: [u8; 3]) -> Self {
        for y in 0..self.imgbuff.height() {
            for x in 0..self.imgbuff.width() {
                self.put_rgb((x, y).into(), color)
                    .expect("Gaurenteed to be in bounds");
            }
        }

        self
    }

    pub fn with_pattern(mut self, p: impl Pattern) -> Self {
        p.draw(&mut self);
        self
    }

    pub fn save<Q: AsRef<Path>>(&self, path: Q) -> ImageResult<()> {
        self.imgbuff.save(path)
    }

    pub fn width(&self) -> u32 {
        self.imgbuff.width()
    }

    pub fn height(&self) -> u32 {
        self.imgbuff.height()
    }
}

// plot
impl Limage {
    pub fn put_rgb(&mut self, pos: Coords, color: [u8; 3]) -> Result<(), LimageError> {
        let (x, y) = pos.convert(self.width(), self.height())?;
        self.imgbuff.put_pixel(x, y, image::Rgb(color));
        Ok(())
    }

    pub fn put_frgb(&mut self, pos: Coords, color: [f32; 3]) -> Result<(), LimageError> {
        let rgb = [
            f32::clamp(color[0] * 255., 0., 255.) as u8,
            f32::clamp(color[1] * 255., 0., 255.) as u8,
            f32::clamp(color[2] * 255., 0., 255.) as u8,
        ];
        self.put_rgb(pos, rgb)
    }

    pub fn put_hsl(&mut self, pos: Coords, hsl: [f32; 3]) -> Result<(), LimageError> {
        let rgb = hsl_to_rgb(hsl);
        self.put_rgb(pos, rgb)
    }
}

// shapes
impl Limage {
    pub fn draw_line(&mut self, p1: Coords, p2: Coords, color: [u8; 3]) {
        let (mut x1, mut y1) = p1.convert_unchecked(self.width(), self.height());
        let (x2, y2) = p2.convert_unchecked(self.width(), self.height());
        let w = x2 - x1;
        let h = y2 - y1;

        let dx1 = match w {
            _ if w < 0 => -1,
            _ if w > 0 => 1,
            _ => 0,
        };
        let dy1 = match h {
            _ if h < 0 => -1,
            _ if h > 0 => 1,
            _ => 0,
        };
        let mut dx2 = dx1;
        let mut dy2 = 0;

        let mut longest = w.abs();
        let mut shortest = h.abs();
        if longest <= shortest {
            longest = h.abs();
            shortest = w.abs();
            dy2 = match h {
                _ if h < 0 => -1,
                _ if h > 0 => 1,
                _ => 0,
            };
            dx2 = 0;
        }

        let mut numerator = longest >> 1;
        for _ in 0..=longest {
            // TODO: Add logging to this?
            self.put_rgb((x1 as u32, y1 as u32).into(), color).ok();
            numerator += shortest;
            if numerator >= longest {
                numerator -= longest;
                x1 += dx1;
                y1 += dy1;
            } else {
                x1 += dx2;
                y1 += dy2;
            }
        }
    }

    pub fn draw_circle(&mut self, center: Coords, r: i32, color: [u8; 3]) {
        let (x, y) = center.convert_unchecked(self.width(), self.height());
        let r = r as i64;

        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b <= r * r {
                    self.put_rgb((ix as u32, iy as u32).into(), color).ok();
                }
            }
        }
    }

    pub fn draw_circle2(&mut self, center: Coords, r: i32, color: [u8; 3]) {
        let (x, y) = center.convert_unchecked(self.width(), self.height());
        let r = r as i64;

        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b < r * (r + 1) {
                    self.put_rgb((ix as u32, iy as u32).into(), color).ok();
                }
            }
        }
    }

    pub fn draw_circle3(&mut self, center: Coords, r: i32, color: [u8; 3]) {
        let (x, y) = center.convert_unchecked(self.width(), self.height());
        let r = r as i64;

        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b <= r * (r + 1) {
                    self.put_rgb((ix as u32, iy as u32).into(), color).ok();
                }
            }
        }
    }

    pub fn draw_circle4(&mut self, center: Coords, r: i32, color: [u8; 3]) {
        let (x, y) = center.convert_unchecked(self.width(), self.height());
        let r = r as i64;

        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b < (r + 1) * (r + 1) {
                    self.put_rgb((ix as u32, iy as u32).into(), color).ok();
                }
            }
        }
    }

    pub fn draw_rectangle(&mut self, topleft: Coords, bottomright: Coords, color: [u8; 3]) {
        let (x1, y1) = topleft.convert_unchecked(self.width(), self.height());
        let (x2, y2) = bottomright.convert_unchecked(self.width(), self.height());

        for x in x1..=x2 {
            for y in y1..=y2 {
                if x >= 0 && y >= 0 {
                    self.put_rgb((x as u32, y as u32).into(), color).ok();
                }
            }
        }
    }
}

// range (360 1 1)
pub fn hsl_to_rgb(hsl: [f32; 3]) -> [u8; 3] {
    let h = hsl[0] % 360.0;
    let s = hsl[1];
    let l = hsl[2];

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let rgb_tmp = match h {
        _ if h < 60.0 => (c, x, 0.0),
        _ if h < 120.0 => (x, c, 0.0),
        _ if h < 180.0 => (0.0, c, x),
        _ if h < 240.0 => (0.0, x, c),
        _ if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((rgb_tmp.0 + m) * 255.999) as u8,
        ((rgb_tmp.1 + m) * 255.999) as u8,
        ((rgb_tmp.2 + m) * 255.999) as u8,
    ]
}

pub const RED: [u8; 3] = [255, 0, 0];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const BLUE: [u8; 3] = [0, 0, 255];
pub const YELLOW: [u8; 3] = [255, 255, 0];
pub const MAGENTA: [u8; 3] = [255, 0, 255];
pub const CYAN: [u8; 3] = [0, 255, 255];

pub const BEIGE: [u8; 3] = [222, 184, 135];
pub const FOREST_GREEN: [u8; 3] = [34, 139, 34];
