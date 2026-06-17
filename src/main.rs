mod operations;
mod point;
mod constants;
mod chaum_pedersen;

use crate::constants::*;
use crate::chaum_pedersen::*;

fn main() {
    let statement = Statement {
        g: G,
        x: X,
        r: R,
        z: Z,
    };

    let commitment = Commitment {
        a: A,
        b: B,
    };

    let candidates = vec![
        83299530,
        83299531,
        83299532,
        83299533,
        83299534,
        83299535,
        83299536,
        83299537,
        83299538,
        83299539,
    ];

    for t in candidates {
        println!("{t}: {}", verify(&statement, &commitment, C, t));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::operations::{mod_mul, mod_add};

    fn is_on_curve(point: &Point) -> bool {
        if point.infinity {
            return true;
        }

        let left = mod_mul(point.y, point.y, P);

        let x2 = mod_mul(point.x, point.x, P);
        let x3 = mod_mul(x2, point.x, P);

        let right = mod_add(mod_add(x3, mod_mul(CURVE_A, point.x, P), P), CURVE_B, P);

        left == right
    }

    #[test]
    fn points_are_on_curve() {
        assert!(is_on_curve(&G));
        assert!(is_on_curve(&X));
        assert!(is_on_curve(&R));
        assert!(is_on_curve(&Z));
        assert!(is_on_curve(&A));
        assert!(is_on_curve(&B));
    }

    #[test]
    fn identity_element() {
        assert_eq!(G.add(&Point::infinity()), G);
        assert_eq!(Point::infinity().add(&G), G);
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
        assert!(G.scalar_mult(NUM_POINTS).infinity);
    }

    #[test]
    fn scalar_mult_basic() {
        assert_eq!(G.scalar_mult(2), G.double());
        assert_eq!(G.scalar_mult(3), G.double().add(&G));
        assert!(G.scalar_mult(NUM_POINTS).infinity);
    }
}
