use crate::core::Limage;

pub trait Pattern {
    fn draw(self, img: &mut Limage);
}

// TODO: Different alignments on grid
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Grid {
    width: u32,
    x_skip: u32,
    y_skip: u32,
    color: [u8; 3],
}

impl Grid {
    pub fn new(width: u32, x_skip: u32, y_skip: u32) -> Self {
        Self {
            width,
            x_skip,
            y_skip,
            color: [0; 3],
        }
    }

    pub fn with_color(mut self, color: [u8; 3]) -> Self {
        self.color = color;
        self
    }
}

impl Pattern for Grid {
    fn draw(self, img: &mut Limage) {
        let (width, height) = (img.width(), img.height());

        let mut x = self.x_skip + self.width / 2;
        while x < width {
            for offset in -(self.width as i64) / 2..=(self.width as i64) / 2 {
                img.draw_line(
                    (x as i64 + offset, 0).into(),
                    (x as i64 + offset, height as i64 - 1).into(),
                    self.color,
                );
            }
            x += self.x_skip;
        }

        let mut y = self.y_skip + self.width / 2;
        while y < height {
            for offset in -(self.width as i64) / 2..=(self.width as i64) / 2 {
                img.draw_line(
                    (0, y as i64 + offset).into(),
                    (width as i64 - 1, y as i64 + offset).into(),
                    self.color,
                );
            }
            y += self.y_skip;
        }
    }
}
