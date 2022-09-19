use std::collections::VecDeque;

pub struct Rectangle {
    pos_x: i32,
    width: i32,
    height: i32,
    step_x: i32,
    step_y: i32,
    x: i32,
    y: i32,
    finished: bool
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
            finished: false
        }
    }
}
impl Iterator for Rectangle {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let p = (self.x, self.y);
        if self.x == self.width {
            if self.y == self.height {
                self.finished = true;
            }
            self.y += self.step_y;
            self.x = self.pos_x;
        }
        else {
            self.x += self.step_x;
        }
        Some(p)
    }
}

pub struct Circle {
    last_pos: (i32, i32),
    last_i: i32,
    origin: (i32, i32),
    radius: i32,
    is_done: bool
}
impl Circle {
    pub fn new(origin: (i32, i32), radius: u32) -> Self {
        Circle {last_pos: (origin.0 - 1, origin.1 - radius as i32), last_i: 1, origin: origin, radius: radius as i32, is_done: false }
    }
}
impl Iterator for Circle {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        if self.radius == 0 {
            self.is_done = true;
            return Some(self.origin);
        }
        let pos = vec![
            (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)
            ];
        let mut pos_iter = pos.iter().cycle();
        for _ in 0..(self.last_i % 8) {
            let _ = pos_iter.next();
        }
        loop {
            let coord: &(i32, i32) = pos_iter.next().unwrap();
            let coord = (coord.0 + self.last_pos.0, coord.1 + self.last_pos.1);
            let c = (coord.0 - self.origin.0, coord.1 - self.origin.1);
            self.last_i += 1;
            if c.0 * c.0 + c.1 * c.1 <= self.radius * (1 + self.radius) {
                if coord.0 == self.origin.0 - 1 && coord.1 == self.origin.1 - self.radius {
                    self.is_done = true;
                }
                self.last_i -= 2;
                self.last_pos = coord;
                return Some(coord);
            }
        }
    }
}

pub struct Disc {
    bounds: Rectangle,
    origin: (i32, i32),
    radius: i32,
}
impl Disc {
    pub fn new(x: i32, y: i32, radius: i32) -> Self {
        Disc {
            bounds: Rectangle::new(x - radius, y - radius, x + radius + 1, y + radius + 1),
            origin: (x, y),
            radius,
        }
    }
}
impl Iterator for Disc {
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
    pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
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

        let numerator = longest >> 1;
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

pub struct Path {
    points: VecDeque<(i32, i32)>,
    last_point: Option<(i32, i32)>,
    curr_line: Line
}
impl Path {
    pub fn new(points: &Vec<(i32, i32)>) -> Self {
        if points.is_empty() {
            panic!("why the fuck would you input an empty list of points");
        }
        if points.len() == 1 {
            return Path {
                points: VecDeque::from(vec![points[0], points[0]]),
                last_point: None,
                curr_line: Line::new(points[0], points[0])
            }
        }
        return Path {
                points: VecDeque::from(points.clone()),
                last_point: None,
                curr_line: Line::new(points[0], points[1])
            }
    }
}
impl Iterator for Path {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.curr_line.next() {
                Some(p) => {
                    match self.last_point {
                        Some(q) => {
                            if p == q {
                                continue;
                            }
                            self.last_point = Some(p);
                            return Some(p);
                        },
                        None => {return Some(p);}
                    }
                }
                None => {
                    loop {
                        if self.points.len() <= 2 {
                            return None;
                        }
                        self.points.remove(0);
                        let p1 = self.points[0];
                        let p2 = self.points[1];
                        if p1 == p2 {
                            continue;
                        }
                        else {
                            self.curr_line = Line::new(p1, p2);
                            let _ = self.curr_line.next();
                            break;
                        }
                    }
                }
            }
        }
    }
}