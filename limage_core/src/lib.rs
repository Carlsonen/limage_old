use image::{ImageBuffer, ImageResult, Pixel, RgbImage, RgbaImage, imageops::{FilterType, self}};

pub trait Limage {
    type ImgType;

    fn new(width: u32, height: u32) -> Self::ImgType;

    fn open(path: &str) -> Result<Self::ImgType, String>;

    fn with_color(self, color: [u8; 3]) -> Self::ImgType;

    fn save(&self, path: &str) -> ImageResult<()>;

    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn in_bounds(&self, p: (i32, i32)) -> bool {
        let (x, y) = p;
        let (w, h) = (self.width() as i32, self.height() as i32);

        x >= 0 && y >= 0 && x < w && y < h
    }

    fn put_rgb(&mut self, p: (i32, i32), color: [u8; 3]);

    fn get_rgb(&self, p: (i32, i32)) -> Option<[u8; 3]>;

    fn paste(&mut self, position: (i32, i32), other: &Self);

    fn resize_to(&mut self, width: u32, height: u32);

    fn as_rgb_buf(&self) -> Vec<u8>;

    fn draw_rectangle(&mut self, p1: (i32, i32), p2: (i32, i32), color: [u8; 3]) {
        for p in shiterators::Rectangle::new(p1, p2) {
            self.put_rgb(p, color);
        }
    }
    fn draw_circle(&mut self, origin: (i32, i32), radius: u32, color: [u8; 3]) {
        for p in shiterators::Circle::new(origin, radius) {
            self.put_rgb(p, color);
        }
    }
    fn draw_disc(&mut self, origin: (i32, i32), radius: u32, color: [u8; 3]) {
        for p in shiterators::Disc::new(origin, radius) {
            self.put_rgb(p, color);
        }
    }
    fn draw_line(&mut self, p1: (i32, i32), p2: (i32, i32), color: [u8; 3]) {
        for p in shiterators::Line::new(p1, p2) {
            self.put_rgb(p, color);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimageRgb {
    pub imgbuff: RgbImage,
}

impl Limage for LimageRgb {
    type ImgType = LimageRgb;

    fn new(width: u32, height: u32) -> Self::ImgType {
        Self { imgbuff: ImageBuffer::new(width, height) }
    }

    fn open(path: &str) -> Result<Self::ImgType, String> {
        match image::open(path) {
            Ok(img) => Ok(Self { imgbuff: img.into_rgb8() }),
            Err(e) => Err(e.to_string())
        }
    }

    fn with_color(mut self, color: [u8; 3]) -> Self::ImgType {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.imgbuff.put_pixel(x, y, image::Rgb(color));
            }
        }

        self
    }

    fn save(&self, path: &str) -> ImageResult<()> {
        self.imgbuff.save(path)
    }

    fn width(&self) -> u32 {
        self.imgbuff.width()
    }

    fn height(&self) -> u32 {
        self.imgbuff.height()
    }

    fn put_rgb(&mut self, p: (i32, i32), color: [u8; 3]) {
        if self.in_bounds(p) {
            let (x, y) = p;
            self.imgbuff
                .put_pixel(x as u32, y as u32, image::Rgb(color));
        }
    }

    fn get_rgb(&self, p: (i32, i32)) -> Option<[u8; 3]> {
        if self.in_bounds(p) {
            let (x, y) = p;
            let rgb = self.imgbuff.get_pixel(x as u32, y as u32).0;
            return Some([rgb[0], rgb[1], rgb[2]]);
        }
        None
    }

    fn paste(&mut self, position: (i32, i32), other: &Self) {
        for y in 0..other.height() as i32 {
            for x in 0..other.width() as i32 {
                let pos = (position.0 + x, position.1 + y);
                if self.in_bounds(pos) {
                    let color = other.get_rgb((x, y)).unwrap();
                    self.put_rgb(pos, color);
                }
            }
        }
    }

    fn as_rgb_buf(&self) -> Vec<u8> {
        self.imgbuff.to_vec()
    }

    fn resize_to(&mut self, width: u32, height: u32) {
        self.imgbuff = imageops::resize(&self.imgbuff, width, height, FilterType::Lanczos3)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LimageRgba {
    pub imgbuff: RgbaImage,
}

impl LimageRgba {
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
}

impl Limage for LimageRgba {
    type ImgType = LimageRgba;

    fn new(width: u32, height: u32) -> Self::ImgType {
        Self { imgbuff: ImageBuffer::new(width, height) }
    }

    fn open(path: &str) -> Result<Self::ImgType, String> {
        match image::open(path) {
            Ok(img) => Ok(Self { imgbuff: img.into_rgba8() }),
            Err(e) => Err(e.to_string())
        }
    }

    fn with_color(mut self, color: [u8; 3]) -> Self::ImgType {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.imgbuff.put_pixel(x, y, image::Rgb(color).to_rgba());
            }
        }

        self
    }

    fn save(&self, path: &str) -> ImageResult<()> {
        self.imgbuff.save(path)
    }

    fn width(&self) -> u32 {
        self.imgbuff.width()
    }

    fn height(&self) -> u32 {
        self.imgbuff.height()
    }

    fn put_rgb(&mut self, p: (i32, i32), color: [u8; 3]) {
        if self.in_bounds(p) {
            let (x, y) = p;
            self.imgbuff
                .put_pixel(x as u32, y as u32, image::Rgb(color).to_rgba());
        }
    }

    fn get_rgb(&self, p: (i32, i32)) -> Option<[u8; 3]> {
        if self.in_bounds(p) {
            let (x, y) = p;
            let rgba = self.imgbuff.get_pixel(x as u32, y as u32).0;
            return Some([rgba[0], rgba[1], rgba[2]]);
        }
        None
    }

    fn paste(&mut self, position: (i32, i32), other: &Self) {
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

    fn as_rgb_buf(&self) -> Vec<u8> {
        let mut bruh = vec![];
        let buf = self.imgbuff.as_raw();
        for (i, b) in buf.iter().enumerate() {
            if i % 4 != 3 {
                bruh.push(*b);
            }
        } 
        bruh
    }

    fn resize_to(&mut self, width: u32, height: u32) {
        self.imgbuff = imageops::resize(&self.imgbuff, width, height, FilterType::Lanczos3)
    }
}
