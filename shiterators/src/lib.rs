use image::RgbaImage;
use std::collections::VecDeque;

pub struct Rectangle {
    pos_x: i32,
    width: i32,
    height: i32,
    step_x: i32,
    step_y: i32,
    x: i32,
    y: i32,
    finished: bool,
}

impl Rectangle {
    pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        let (x, y) = p1;
        let (w, h) = p2;
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
            finished: false,
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
        } else {
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
    is_done: bool,
}

impl Circle {
    pub fn new(origin: (i32, i32), radius: u32) -> Self {
        Circle {
            last_pos: (origin.0 - 1, origin.1 - radius as i32),
            last_i: 1,
            origin: origin,
            radius: radius as i32,
            is_done: false,
        }
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
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
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
    pub fn new(origin: (i32, i32), radius: u32) -> Self {
        let radius = radius as i32;
        let (x, y) = origin;
        Disc {
            bounds: Rectangle::new((x - radius, y - radius), (x + radius + 1, y + radius + 1)),
            origin: origin,
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
    finished: bool,
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
            finished: false,
        }
    }
}

impl Iterator for Line {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
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


pub struct PathCircuit {
    points: VecDeque<(i32, i32)>,
    last_point: Option<(i32, i32)>,
    curr_line: Line,
    is_circuit: bool,
    start: (i32, i32),
}

impl PathCircuit {
    pub fn new_path(points: &Vec<(i32, i32)>) -> Self {
        if points.is_empty() {
            panic!("why the fuck would you input an empty list of points");
        }
        if points.len() == 1 {
            return PathCircuit {
                points: VecDeque::from(vec![points[0], points[0]]),
                last_point: None,
                curr_line: Line::new(points[0], points[0]),
                is_circuit: false,
                start: (0, 0) // doesnt matter
            };
        }
        return PathCircuit {
            points: VecDeque::from(points.clone()),
            last_point: None,
            curr_line: Line::new(points[0], points[1]),
            is_circuit: false,
            start: (0, 0) // doesnt matter
        };
    }
    pub fn new_circuit(points: &Vec<(i32, i32)>) -> Self {
        if points.is_empty() {
            panic!("why the fuck would you input an empty list of points");
        }
        if points.len() == 1 {
            return PathCircuit {
                points: VecDeque::from(vec![points[0], points[0]]),
                last_point: None,
                curr_line: Line::new(points[0], points[0]),
                is_circuit: false,
                start: (0, 0) // doesnt matter
            };
        }
        let mut new_points = points.clone();
        new_points.push(points[0]);
        return PathCircuit {
            points: VecDeque::from(new_points.clone()),
            last_point: None,
            curr_line: Line::new(new_points[0], new_points[1]),
            is_circuit: true,
            start: points[0].clone()
        };
    }
}

impl Iterator for PathCircuit {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.curr_line.next() {
                Some(p) => match self.last_point {
                    Some(q) => {
                        if p == q {
                            continue;
                        }
                        if self.is_circuit && self.points.len() <= 2 && p == self.start {
                            return None;
                        }
                        self.last_point = Some(p);
                        return Some(p);
                    }
                    None => {
                        self.last_point = Some(p);
                        return Some(p);
                    }
                },
                None => loop {
                    if self.points.len() <= 2 {
                        return None;
                    }
                    self.points.remove(0);
                    let p1 = self.points[0];
                    let p2 = self.points[1];
                    if p1 == p2 {
                        continue;
                    } else {
                        self.curr_line = Line::new(p1, p2);
                        let _ = self.curr_line.next();
                        break;
                    }
                },
            }
        }
    }
}

pub struct Text {
    position: (i32, i32),
    text: String,
    size: i32,
    font_sheet: RgbaImage,
    current_index: usize,
    current_box: Rectangle,
}

impl Text {
    pub fn new(position: (i32, i32), text: &str, size: u32) -> Self {
        Text {
            position: position,
            font_sheet: image::open("assets/font.png").unwrap().into_rgba8(),
            text: text.to_string(),
            size: size as i32,
            current_index: 0,
            current_box: Rectangle::new((0, 0), (6 * size as i32 - 1, 12 * size as i32 - 1))
        }
    }
}

impl Iterator for Text {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index == self.text.len() {
            return None;
        }
        
        loop {
            let c = self.text.as_bytes()[self.current_index];

            let char_index = match c {
                c if c >= 32 && c <= 126 => {
                    (c - 32) as i32
                }
                c if c >= 161 => {
                    0
                } 
                _ => {
                    0
                }
            };
            match self.current_box.next() {
                Some((x, y)) => {
                    let pos = (x / self.size + 6 * (char_index % 21), y / self.size + 12 * (char_index / 21));
                    let sample = self.font_sheet.get_pixel(pos.0 as u32, pos.1 as u32).0;
                    if sample == [255; 4] {
                        let shit = (self.position.0 + self.size * 6 * self.current_index as i32 + x, self.position.1 + y);
                        return Some(shit);
                    }
                    else {
                        continue;
                    }
                }
                None => {
                    self.current_index += 1;
                    if self.current_index == self.text.len() {
                        return None;
                    }
                    self.current_box = Rectangle::new((0, 0), (6 * self.size - 1, 12 * self.size - 1));
                }
            }
        }
    }
}

pub struct WireFrame {
    vertex_table: Vec<(i32, i32)>,
    edge_table: Vec<(usize, usize)>,
    curr_edge: usize,
    curr_line: Line,
}

impl WireFrame {
    pub fn new(vertex_table: &Vec<(i32, i32)>, edge_table: &Vec<(usize, usize)>) -> Self {
        if edge_table.is_empty() {
            panic!("no edges");
        }
        let (e1, e2) = edge_table[0];
        WireFrame { vertex_table: vertex_table.clone(), edge_table: edge_table.clone(), curr_edge: 0, curr_line: Line::new(vertex_table[e1], vertex_table[e2]) }
    }
}

impl Iterator for WireFrame {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_line.next() {
            Some(p) => {Some(p)},
            None => {
                self.curr_edge += 1;
                if self.curr_edge >= self.edge_table.len() {
                    return None;
                }
                let (e1, e2) = self.edge_table[self.curr_edge];
                self.curr_line = Line::new(self.vertex_table[e1], self.vertex_table[e2]);

                self.next()
            }
        }
    }
}