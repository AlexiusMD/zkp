use crate::constants::*;
use crate::operations::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}
impl Point { pub fn new(x: u64, y: u64, infinity: bool) -> Point {
        Point { x, y, infinity }
    }

    pub fn infinity() -> Point {
        Point {
            x: 0,
            y: 0,
            infinity: true,
        }
    }

    pub fn add(&self, other: &Point) -> Point {
        if self.infinity {
            return *other;
        }

        if other.infinity {
            return *self;
        }

        // P + (-P) = O
        if self.x == other.x && mod_add(self.y, other.y, P) == 0 {
            return Point::infinity();
        }

        // P + P
        if self == other {
            return self.double();
        }

        let numerator = mod_sub(other.y, self.y, P);
        let denominator = mod_sub(other.x, self.x, P);

        let lambda = mod_div(numerator, denominator, P);

        let x3 = mod_sub(mod_sub(mod_mul(lambda, lambda, P), self.x, P), other.x, P);
        let y3 = mod_sub(mod_mul(lambda, mod_sub(self.x, x3, P), P), self.y, P);

        Point::new(x3, y3, false)
    }

    pub fn double(&self) -> Point {
        if self.infinity {
            return Point::infinity();
        }

        if self.y == 0 {
            return Point::infinity();
        }

        let x_squared = mod_mul(self.x, self.x, P);

        let numerator = mod_add(mod_mul(3, x_squared, P), CURVE_A as u64, P);
        let denominator = mod_mul(2, self.y, P);

        let lambda = mod_div(numerator, denominator, P);

        let x3 = mod_sub(mod_mul(lambda, lambda, P), mod_mul(2, self.x, P), P);
        let y3 = mod_sub(mod_mul(lambda, mod_sub(self.x, x3, P), P), self.y, P);

        Point::new(x3, y3, false)
    }

    // Double and add algorithm
    pub fn scalar_mult(&self, scalar: u64) -> Point {
        if scalar == 0 {
            return Point::infinity();
        }

        let mut result = Point::infinity();

        let most_significant_bit = 63 - scalar.leading_zeros();

        // Iterating from most significant bit to least significant
        for _i in (0..=most_significant_bit).rev() {
            result = result.double();

            if ((scalar >> 1) & 1) == 1 {
                result = result.add(self);
            }
        }

        result
    }

    pub fn neg(&self) -> Point {
        if self.infinity {
            return Point::infinity();
        }

        Point::new(self.x, mod_neg(self.y, P), false)
    }

    pub fn is_on_curve(&self) -> bool {
        if self.infinity {
            return true;
        }

        let left = mod_mul(self.y, self.y, P);

        let x2 = mod_mul(self.x, self.x, P);
        let x3 = mod_mul(x2, self.x, P);

        let right = mod_add(mod_add(x3, mod_mul(CURVE_A, self.x, P), P), CURVE_B, P);

        left == right
    }
}
