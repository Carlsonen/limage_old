use image::{ImageBuffer, ImageResult, RgbImage};

mod shape_iterators;
use shape_iterators::*;

pub fn rectangle(x: i32, y: i32, w: i32, h: i32) -> Rectangle {
    Rectangle::new(x, y, w, h)
}

pub struct Limage {
    pub imgbuff: RgbImage,
}

// make - save
impl Limage {
    pub fn new(width: u32, height: u32) -> Self {
        Limage {
            imgbuff: ImageBuffer::new(width, height),
        }
    }
    pub fn from_color(width: u32, height: u32, color: [u8; 3]) -> Self {
        let mut img = Limage::new(width, height);
        for y in 0..img.height() {
            for x in 0..img.width() {
                img.put_rgb(x, y, color);
            }
        }

        img
    }
    pub fn save(&self, path: &str) -> ImageResult<()> {
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
    pub fn put_rgb(&mut self, x: u32, y: u32, color: [u8; 3]) {
        if x < self.imgbuff.width() && y < self.imgbuff.height() {
            self.imgbuff.put_pixel(x, y, image::Rgb(color));
        }
    }
    pub fn put_frgb(&mut self, x: u32, y: u32, color: [f32; 3]) {
        if x < self.imgbuff.width() && y < self.imgbuff.height() {
            let rgb = [
                (color[0] * 255.999) as u8,
                (color[1] * 255.999) as u8,
                (color[2] * 255.999) as u8,
            ];
            self.imgbuff.put_pixel(x, y, image::Rgb(rgb));
        }
    }
    pub fn put_hsl(&mut self, x: u32, y: u32, hsl: [f32; 3]) {
        let rgb = hsl_to_rgb(hsl);
        self.put_rgb(x, y, rgb);
    }
}

// shapes
impl Limage {
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: [u8; 3]) {
        let mut x1 = x1;
        let mut y1 = y1;
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
        if !(longest > shortest) {
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
            self.put_rgb(x1 as u32, y1 as u32, color);
            numerator += shortest;
            if !(numerator < longest) {
                numerator -= longest;
                x1 += dx1;
                y1 += dy1;
            } else {
                x1 += dx2;
                y1 += dy2;
            }
        }
    }
    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: [u8; 3]) {
        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b <= r * r {
                    self.put_rgb(ix as u32, iy as u32, color);
                }
            }
        }
    }
    pub fn draw_circle2(&mut self, x: i32, y: i32, r: i32, color: [u8; 3]) {
        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b < r * (r + 1) {
                    self.put_rgb(ix as u32, iy as u32, color);
                }
            }
        }
    }
    pub fn draw_circle3(&mut self, x: i32, y: i32, r: i32, color: [u8; 3]) {
        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b <= r * (r + 1) {
                    self.put_rgb(ix as u32, iy as u32, color);
                }
            }
        }
    }
    pub fn draw_circle4(&mut self, x: i32, y: i32, r: i32, color: [u8; 3]) {
        for a in -r..=r {
            for b in -r..=r {
                let ix = x + a;
                let iy = y + b;
                if ix >= 0 && iy >= 0 && a * a + b * b < (r + 1) * (r + 1) {
                    self.put_rgb(ix as u32, iy as u32, color);
                }
            }
        }
    }
    pub fn draw_rectangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: [u8; 3]) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                if x >= 0 && y >= 0 {
                    self.put_rgb(x as u32, y as u32, color);
                }
            }
        }
    }
}
impl Limage {
    pub fn fdraw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: [u8; 3]) {
        self.draw_line(
            (x1 * self.width() as f32 - 0.001) as i32,
            (y1 * self.height() as f32 - 0.001) as i32,
            (x2 * self.width() as f32 - 0.001) as i32,
            (y2 * self.height() as f32 - 0.001) as i32,
            color,
        )
    }
    pub fn fdraw_circle(&mut self, x: f32, y: f32, r: f32, color: [u8; 3]) {
        self.draw_circle(
            (x * self.width() as f32 - 0.001) as i32,
            (y * self.height() as f32 - 0.001) as i32,
            (r * self.height() as f32 - 0.001) as i32,
            color,
        )
    }
    pub fn fdraw_rectangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: [u8; 3]) {
        self.draw_rectangle(
            (x1 * self.width() as f32 - 0.001) as i32,
            (y1 * self.width() as f32 - 0.001) as i32,
            (x2 * self.width() as f32 - 0.001) as i32,
            (y2 * self.width() as f32 - 0.001) as i32,
            color,
        )
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

pub const RED: [u8; 3] = [255,0,0];
pub const GREEN: [u8; 3] = [0,255,0];
pub const BLUE: [u8; 3] = [0,0,255];
pub const YELLOW: [u8; 3] = [255,255,0];
pub const MAGENTA: [u8; 3] = [255,0,255];
pub const CYAN: [u8; 3] = [0,255,255];

pub const BEIGE: [u8; 3] = [222,184,135];
pub const FOREST_GREEN: [u8; 3] = [34, 139, 34];