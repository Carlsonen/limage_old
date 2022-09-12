


pub struct Rectangle {
    pos_x: i32,
    width: i32,
    height: i32,
    step_x: i32,
    step_y: i32,
    x: i32,
    y: i32
}
impl Rectangle {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let step_x = match w {
            _ if w < 0 => -1,
            _ => 1
        };
        let step_y = match h {
            _ if h < 0 => -1,
            _ => 1
        };
    
        Rectangle { pos_x: x, width: w, height: h, step_x: step_x, step_y: step_y, x: x, y: y }
    }
}
impl Iterator for Rectangle {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {    
        self.x += self.step_x;
        if self.x == self.width {
            self.y += self.step_y;
            if self.y == self.height {
                return None
            }
            self.x = self.pos_x;
        }
        
        Some((self.x, self.y))
    }
}