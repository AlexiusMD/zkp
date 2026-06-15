mod operations;
mod point;
mod constants;
mod chaum_pedersen;

use crate::constants::*;
use crate::point::Point;
use crate::chaum_pedersen::*;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_are_on_curve() {
        assert!(G.is_on_curve());
        assert!(X.is_on_curve());
        assert!(R.is_on_curve());
        assert!(Z.is_on_curve());
        assert!(A.is_on_curve());
        assert!(B.is_on_curve());
    }

    #[test]
    fn identity_element() {
        assert_eq!(G.add(&Point::infinity()), G);
        assert_eq!(Point::infinity().add(&G), G);
    }

    #[test]
    fn additive_inverse() {
        assert!(G.add(&G.neg()).infinity);
    }

    #[test]
    fn point_doubling() {
        assert_eq!(G.add(&G), G.double());
    }

    #[test]
    fn scalar_multiplication() {
        assert_eq!(G.scalar_mult(3), G.double().add(&G));
    }

    #[test]
    fn generator_order() {
        assert!(G.scalar_mult(Q).infinity);
    }
}
