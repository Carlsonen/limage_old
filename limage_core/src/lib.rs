use image::{
    imageops::{self, FilterType},
    DynamicImage, ImageBuffer, ImageResult, Pixel, RgbImage, Rgba, RgbaImage,
};

use imageproc::drawing::{draw_text_mut, text_size};
use rand;
use reqwest;
use rusttype::{Font, Scale};
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

    fn as_resized(self, width: u32, height: u32) -> Self;

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
        Self {
            imgbuff: ImageBuffer::new(width, height),
        }
    }

    fn open(path: &str) -> Result<Self::ImgType, String> {
        match image::open(path) {
            Ok(img) => Ok(Self {
                imgbuff: img.into_rgb8(),
            }),
            Err(e) => Err(e.to_string()),
        }
    }

    fn with_color(mut self, color: [u8; 3]) -> Self::ImgType {
        self.imgbuff.pixels_mut().for_each(|p| {
            *p = image::Rgb(color);
        });
        self
    }
    #[inline]
    fn save(&self, path: &str) -> ImageResult<()> {
        self.imgbuff.save(path)
    }
    #[inline]
    fn width(&self) -> u32 {
        self.imgbuff.width()
    }
    #[inline]
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
            return Some(rgb);
        }
        None
    }

    fn paste(&mut self, position: (i32, i32), other: &Self) {
        imageops::overlay(
            &mut self.imgbuff,
            &other.imgbuff,
            position.0 as i64,
            position.1 as i64,
        );
    }

    #[inline]
    fn as_rgb_buf(&self) -> Vec<u8> {
        self.imgbuff.to_vec()
    }

    fn as_resized(mut self, width: u32, height: u32) -> Self {
        self.imgbuff = imageops::resize(&self.imgbuff, width, height, FilterType::Lanczos3);
        self
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
        Self {
            imgbuff: ImageBuffer::new(width, height),
        }
    }

    fn open(path: &str) -> Result<Self::ImgType, String> {
        match image::open(path) {
            Ok(img) => Ok(Self {
                imgbuff: img.into_rgba8(),
            }),
            Err(e) => Err(e.to_string()),
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
        imageops::overlay(
            &mut self.imgbuff,
            &other.imgbuff,
            position.0 as i64,
            position.1 as i64,
        );
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

    fn as_resized(mut self, width: u32, height: u32) -> Self {
        self.imgbuff = imageops::resize(&self.imgbuff, width, height, FilterType::Lanczos3);
        self
    }
}

impl LimageRgba {
    pub fn write_text(
        &mut self,
        pos: (i32, i32),
        color: [u8; 4],
        text: &str,
        size: f32,
        font: &str,
    ) {
        let font = std::fs::read(format!("./assets/{font}")).unwrap();
        let font = Font::try_from_vec(font).unwrap();
        let scale = Scale {
            x: size * 2.0,
            y: size,
        };
        draw_text_mut(
            &mut self.imgbuff,
            Rgba(color),
            pos.0,
            pos.1,
            scale,
            &font,
            text,
        );
    }
}

pub fn sizeof_text(text: &str, size: f32, font: &str) -> (i32, i32) {
    let font = std::fs::read(format!("./assets/{font}")).unwrap();
    let font = Font::try_from_vec(font).unwrap();
    let scale = Scale {
        x: size * 2.0,
        y: size,
    };
    text_size(scale, &font, text)
}

pub enum PexelMode {
    Original,
    Landscape,
    Portrait,
}

pub fn from_google(query: &str, mode: PexelMode) -> DynamicImage {
    let pexels_api_client =
        pexels::Pexels::new("kWALdmnm5cdOAOU08nEEhrFd8tdjno4QdA5bda7LuXvH2JL04AV4ebnT".to_owned());
    let shit = pexels_api_client.photo_search(query.to_string(), 256, 1);
    let n = &shit["photos"].as_array().unwrap().len();
    if *n == 0 {
        panic!("found no image with query '{}'", query)
    }
    let i: usize = rand::random::<usize>() % n;
    let mode = match mode {
        PexelMode::Original => "original",
        PexelMode::Landscape => "landscape",
        PexelMode::Portrait => "portrait",
    };
    let url = shit["photos"][i]["src"][mode].as_str().unwrap();
    let data = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let img = image::load_from_memory(&data).unwrap();
    img
}

pub trait ToLimage {
    fn to_limage_rgb(self) -> LimageRgb;
    fn to_limage_rgba(self) -> LimageRgba;
}

impl ToLimage for DynamicImage {
    fn to_limage_rgb(self) -> LimageRgb {
        LimageRgb {
            imgbuff: self.into_rgb8(),
        }
    }
    fn to_limage_rgba(self) -> LimageRgba {
        LimageRgba {
            imgbuff: self.into_rgba8(),
        }
    }
}
