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
    pub fn convert(self, width: u32, height: u32) -> Result<(u32, u32), super::LimageError> {
        match self {
            Self::Whole(x, y) if x >= 0 && (x as u32) < width && y >= 0 && (y as u32) < height => {
                Ok((x as u32, y as u32))
            }
            Self::Fractional(x, y) if (0. ..=1.).contains(&x) && (0. ..=1.).contains(&y) => Ok((
                (x * (width as f32 - 1.)) as u32,
                (y * (height as f32 - 1.)) as u32,
            )),
            _ => Err(super::LimageError::OutOfBounds),
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
