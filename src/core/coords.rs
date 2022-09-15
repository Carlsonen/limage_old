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

#[cfg(test)]
mod test {
    #[test]
    fn accepts_u32() {
        use super::Coords;

        let c: Coords = (1u32, 1u32).into();
        assert_eq!(Coords::Whole(1, 1), c);
    }

    #[test]
    fn accepts_i64() {
        use super::Coords;

        let c: Coords = (1i64, 1i64).into();
        assert_eq!(Coords::Whole(1, 1), c);
    }

    #[test]
    fn accepts_f32() {
        use super::Coords;

        let c: Coords = (1., 1.).into();
        assert_eq!(Coords::Fractional(1., 1.), c);
    }

    #[test]
    fn converts_whole() {
        use super::Coords;

        let c: Coords = (1u32, 1u32).into();
        assert_eq!(c.convert(100, 100), Ok((1u32, 1u32)));
    }

    #[test]
    fn checks_whole_bounds() {
        use super::Coords;
        use crate::core::LimageError;

        let c: Coords = (1000u32, 1000u32).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (100u32, 0u32).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (-1i64, 0i64).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (0u32, 100u32).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (0i64, -1i64).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));
    }

    #[test]
    fn returns_whole_unchecked() {
        use super::Coords;

        let c: Coords = (-1i64, 1i64).into();
        assert_eq!(c.convert_unchecked(100, 100), (-1, 1));

        let c: Coords = (100i64, 0i64).into();
        assert_eq!(c.convert_unchecked(100, 100), (100, 0));

        let c: Coords = (1i64, -1i64).into();
        assert_eq!(c.convert_unchecked(100, 100), (1, -1));

        let c: Coords = (0i64, 100i64).into();
        assert_eq!(c.convert_unchecked(100, 100), (0, 100));
    }

    #[test]
    fn converts_fractional() {
        use super::Coords;

        let c: Coords = (1., 1.).into();
        assert_eq!(c.convert(100, 100), Ok((99, 99)));

        let c: Coords = (0., 0.).into();
        assert_eq!(c.convert(100, 100), Ok((0, 0)));
    }

    #[test]
    fn checks_fractional_bounds() {
        use super::Coords;
        use crate::core::LimageError;

        let c: Coords = (2., 0.).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (-1., 0.).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (0., 2.).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));

        let c: Coords = (0., -1.).into();
        assert_eq!(c.convert(100, 100), Err(LimageError::OutOfBounds));
    }

    #[test]
    fn returns_fractional_unchecked() {
        use super::Coords;

        let c: Coords = (2., 0.).into();
        assert_eq!(c.convert_unchecked(100, 100), (198, 0));

        let c: Coords = (-1., 0.).into();
        assert_eq!(c.convert_unchecked(100, 100), (-99, 0));

        let c: Coords = (0., 2.).into();
        assert_eq!(c.convert_unchecked(100, 100), (0, 198));

        let c: Coords = (0., -1.).into();
        assert_eq!(c.convert_unchecked(100, 100), (0, -99));
    }
}
