const P: u64 = 649942387;
const A: u64 = 2;

pub struct ECCPoint {
    x: u64,
    y: u64,
}

impl ECCPoint {
    pub fn new(x: u64, y: u64) -> ECCPoint {
        ECCPoint { x, y }
    }

    pub fn add_points(&self, other_point: ECCPoint) -> ECCPoint {
        let s = 0;
        let x3 = ((s * s) - self.x - other_point.x) % P;
        let y3 = (s * (self.x - x3) - self.y) % P;

        ECCPoint {
            x: x3,
            y: y3,
        }
    }

    pub fn is_same_point(&self, other_point: &ECCPoint) -> bool {
        (self.x == other_point.x) && (self.y == other_point.y)
    }
}
