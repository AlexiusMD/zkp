use crate::operations::*;
use crate::constants::*;

pub struct Point {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

impl Point {
    pub fn new(x: u64, y: u64, infinity: bool) -> Point {
        Point {
            x,
            y,
            infinity,
        }
    }

    pub fn add(&self, other_point: &Point) -> Point {
        if self.is_same_point(other_point) {
            return self.double();
        }
        let s = mod_inv(mod_sub(other_point.y, self.y, P), mod_sub(other_point.x, self.x, P)) % P;
        let x3 = mod_sub(mod_sub(s * s, self.x, P), other_point.x, P); 
        Point {
            x: x3,
            y: mod_sub(mod_mul(s, mod_sub(self.x, x3, P), P), self.y, P),
            infinity: false,
        }
    }

    pub fn double(&self) -> Point {

    }

    pub fn scalar_mult(&self, scalar: u64) -> Point {

    }

    fn is_same_point(&self, other_point: &Point) -> bool {
        self.x == other_point.x && self.y == other_point.y
    }
}
