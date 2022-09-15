pub struct Rectangle {
    pos_x: i32,
    width: i32,
    height: i32,
    step_x: i32,
    step_y: i32,
    x: i32,
    y: i32,
}
impl Rectangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let step_x = match w {
            _ if w < 0 => -1,
            _ => 1,
        };
        let step_y = match h {
            _ if h < 0 => -1,
            _ => 1,
        };

        Rectangle {
            pos_x: x,
            width: w,
            height: h,
            step_x: step_x,
            step_y: step_y,
            x: x,
            y: y,
        }
    }
}
impl Iterator for Rectangle {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.width {
            self.y += self.step_y;
            if self.y == self.height {
                return None;
            }
            self.x = self.pos_x;
        }
        self.x += self.step_x;
        Some((self.x - 1, self.y))
    }
}

pub struct Circle {
    bounds: Rectangle,
    origin: (i32, i32),
    radius: i32,
}
impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Self {
        Circle {
            bounds: Rectangle::new(x - radius, y - radius, x + radius + 1, y + radius + 1),
            origin: (x, y),
            radius,
        }
    }
}
impl Iterator for Circle {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let coord = match self.bounds.next() {
                Some(coord) => coord,
                None => return None,
            };
            let c = (coord.0 - self.origin.0, coord.1 - self.origin.1);
            if c.0 * c.0 + c.1 * c.1 <= self.radius * (1 + self.radius) {
                return Some(coord);
            }
        }
    }
}
