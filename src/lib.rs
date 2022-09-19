use image::{ImageBuffer, ImageResult, RgbaImage, Pixel};

mod shiterators;
use shiterators::*;

pub fn rectangle(p1: (i32, i32), p2: (i32, i32)) -> Rectangle {
    let (x, y) = p1;
    let (w, h) = p2;
    Rectangle::new(x, y, w, h)
}
pub fn circle(origin: (i32, i32), radius: u32) -> Circle {
    let (x, y) = origin;
    Circle::new((x, y), radius)
}
pub fn disc(origin: (i32, i32), radius: u32) -> Disc {
    let (x, y) = origin;
    Disc::new(x, y, radius as i32)
}
pub fn line(p1: (i32, i32), p2: (i32, i32)) -> Line {
    Line::new(p1, p2)
}
pub fn path(points: &Vec<(i32, i32)>) -> Path {
    Path::new(points)
}

pub struct Limage {
    pub imgbuff: RgbaImage,
}

impl Limage {
    pub fn new(width: u32, height: u32) -> Self {
        Limage {
            imgbuff: ImageBuffer::new(width, height),
        }
    }
    pub fn open(path: &String) -> Result<Self, String> {
        match image::open(path) {
            Ok(file) => {
                let img: RgbaImage = file.into_rgba8();
                return Ok(Limage { imgbuff: img });
            }
            Err(_) => {
                return Err("Could not open file".to_string());
            }
        }
    }
    pub fn with_color(mut self, color: [u8; 3]) -> Self {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.imgbuff.put_pixel(x, y, image::Rgb(color).to_rgba());
            }
        }

        self
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
    pub fn in_bounds(&self, p: (i32, i32)) -> bool {
        let (x, y) = p;
        let (w, h) = (self.width() as i32, self.height() as i32);

        x >= 0 && y >= 0 && x < w && y < h
    }
    pub fn put_rgb(&mut self, p: (i32, i32), color: [u8; 3]) {
        if self.in_bounds(p) {
            let (x, y) = p;
            self.imgbuff.put_pixel(x as u32, y as u32, image::Rgb(color).to_rgba());
        }
    }
    pub fn put_rgba(&mut self, p: (i32, i32), color: [u8; 4]) {
        if self.in_bounds(p) {
            let (x, y) = p;
            self.imgbuff.put_pixel(x as u32, y as u32, image::Rgba(color));
        }
    }
    pub fn get_rgba(&self, p: (i32, i32)) -> Option<[u8; 4]> {
        if self.in_bounds(p) {
            let (x, y) = p;
            return Some(self.imgbuff.get_pixel(x as u32, y as u32).0);
        }
        None
    }
    pub fn paste(&mut self, position: (i32, i32), other: &Limage) {
        for p in rectangle((0, 0), (other.width() as i32 -1, other.height() as i32 -1)) {
            let pos = (position.0 + p.0, position.1 + p.1);
            if self.in_bounds(pos) {
                let old_color = self.get_rgba(pos).unwrap();
                let color = other.get_rgba(p).unwrap();
                let color = blend_color(old_color, color);
                self.put_rgba(pos, color);
            }
        }
    }
}

// straight up wrong but can be fixed another time
pub fn blend_color(old: [u8; 4], new: [u8; 4]) -> [u8; 4] {
    let a = new[3] as f32 / 255.;
    let b = 1. - a;
    let red = b * old[0] as f32 + a * new[0] as f32;
    let green = b * old[1] as f32 + a * new[1] as f32;
    let blue = b * old[2] as f32 + a * new[2] as f32;
    let a = a.max(old[3] as f32 / 255.) * 255.999;
    [red as u8, green as u8, blue as u8, a as u8]
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
