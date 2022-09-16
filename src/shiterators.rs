
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

pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    numerator: i32,
    shortest: i32,
    longest: i32,
    dx1: i32,
    dy1: i32,
    dx2: i32,
    dy2: i32,
    finished: bool
}
impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
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
        Line {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            numerator: numerator,
            shortest: shortest,
            longest: longest,
            dx1: dx1,
            dy1: dy1,
            dx2: dx2,
            dy2: dy2,
            finished: false
        }
        
    }
}
impl Iterator for Line {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {return None;}
        let x = self.x1;
        let y = self.y1;
        self.numerator += self.shortest;
        if !(self.numerator < self.longest) {
            self.numerator -= self.longest;
            self.x1 += self.dx1;
            self.y1 += self.dy1;
        } else {
            self.x1 += self.dx2;
            self.y1 += self.dy2;
        }
        if x == self.x2 && y == self.y2 {
            self.finished = true;
        }
        return Some((x, y));
    }
}