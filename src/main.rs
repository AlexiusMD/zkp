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
        113427546,
        443289651,
        83299537,
        498116895,
        247655242,
        159770415,
    ];

    for t in candidates {
        println!("{t}: {}", verify(&statement, &commitment, C, t));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;

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
