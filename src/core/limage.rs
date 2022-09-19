use super::rectangle;
use crate::blend_color;

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

    pub fn get_rgba(&self, p: (i32, i32)) -> Option<[u8; 4]> {
        if self.in_bounds(p) {
            let (x, y) = p;
            return Some(self.imgbuff.get_pixel(x as u32, y as u32).0);
        }
        None
    }

    pub fn paste(&mut self, position: (i32, i32), other: &Limage) {
        for p in rectangle(
            (0, 0),
            (other.width() as i32 - 1, other.height() as i32 - 1),
        ) {
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
