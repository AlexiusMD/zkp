mod operations;
mod point;
mod constants;

use crate::constants::*;
use crate::point::Point;

fn main() {
    assert!(G.is_on_curve());
    assert!(X.is_on_curve());
    assert!(R.is_on_curve());
    assert!(Z.is_on_curve());
    assert!(A.is_on_curve());
    assert!(B.is_on_curve());
    assert_eq!(G.add(&Point::infinity()), G);
    assert_eq!(Point::infinity().add(&G), G);
    assert!(G.add(&G.neg()).infinity);
    assert_eq!(G.add(&G), G.double());
    assert_eq!(G.scalar_mult(3), G.double().add(&G));
    assert!(G.scalar_mult(Q).infinity);
}
