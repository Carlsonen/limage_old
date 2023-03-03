use shiterators::*;
use image::{ImageBuffer, ImageResult, Pixel, RgbaImage};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Limage {
    pub imgbuff: RgbaImage,
}

impl Limage {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            imgbuff: ImageBuffer::new(width, height),
        }
    }

    pub fn open(path: &str) -> Result<Limage, String> {
        let result = image::open(path);
        match result {
            Ok(file) => {
                return Ok(Limage { imgbuff: file.into_rgba8() });
            }
            Err(_) => {
                return Err(format!("Could not open file {}", path));
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
            self.imgbuff
                .put_pixel(x as u32, y as u32, image::Rgb(color).to_rgba());
        }
    }

    pub fn put_rgba(&mut self, p: (i32, i32), color: [u8; 4]) {
        if self.in_bounds(p) {
            let (x, y) = p;
            self.imgbuff
                .put_pixel(x as u32, y as u32, image::Rgba(color));
        }
    }

    pub fn get_rgb(&self, p: (i32, i32)) -> Option<[u8; 3]> {
        if self.in_bounds(p) {
            let (x, y) = p;
            let rgba = self.imgbuff.get_pixel(x as u32, y as u32).0;
            return Some([rgba[0], rgba[1], rgba[2]]);
        }
        None
    }

    pub fn get_rgba(&self, p: (i32, i32)) -> Option<[u8; 4]> {
        if self.in_bounds(p) {
            let (x, y) = p;
            return Some(self.imgbuff.get_pixel(x as u32, y as u32).0);
        }
        None
    }

    pub fn paste(&mut self, position: (i32, i32), other: &Limage) {
        for y in 0..other.height() as i32 {
            for x in 0..other.width() as i32 {
                let pos = (position.0 + x, position.1 + y);
                if self.in_bounds(pos) {
                    let color = other.get_rgba((x, y)).unwrap();
                    self.put_rgba(pos, color);
                }
            }
        }
    }
}

// shiterators shortcut
impl Limage {
    pub fn draw_rectangle(&mut self, p1: (i32, i32), p2: (i32, i32), color: [u8; 3]) {
        for p in shiterators::Rectangle::new(p1, p2) {
            self.put_rgb(p, color);
        }
    }
    pub fn draw_circle(&mut self, origin: (i32, i32), radius: u32, color: [u8; 3]) {
        for p in shiterators::Circle::new(origin, radius) {
            self.put_rgb(p, color);
        }
    }
    pub fn draw_disc(&mut self, origin: (i32, i32), radius: u32, color: [u8; 3]) {
        for p in shiterators::Disc::new(origin, radius) {
            self.put_rgb(p, color);
        }
    }
    pub fn draw_line(&mut self, p1: (i32, i32), p2: (i32, i32), color: [u8; 3]) {
        for p in shiterators::Line::new(p1, p2) {
            self.put_rgb(p, color);
        }
    }
}
