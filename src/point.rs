#[derive(Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn add(&self, other: Point) -> Point{
        return Point { 
            x: self.x + other.x, 
            y: self.y + other.y
        };
    }

    pub fn from_sum(p1:Point, p2:Point) -> Point{
        return Point { 
            x: p1.x + p2.x, 
            y: p1.y + p2.y
        };
    }
}