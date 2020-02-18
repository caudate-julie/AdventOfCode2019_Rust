use std::ops;


#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
#[derive(Hash)]
#[derive(Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ops::Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord { 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<i32> for Coord {
    type Output = Coord;
    fn mul(self, other: i32) -> Coord {
        Coord { 
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Div<i32> for Coord {
    type Output = Coord;
    fn div(self, other: i32) -> Coord {
        assert!(other != 0 && self.x % other == 0 && self.y % other == 0);
        Coord { 
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Coord {
    pub fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord { x, y, z }
    }
}