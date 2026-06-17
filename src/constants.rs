use crate::point::Point;

// Prime and number of points
pub const P: u64 = 649_942_387;
pub const _NUM_POINTS: u64 = 649_919_997;

// Point G
pub const G: Point = Point {
    x: 127_310_624,
    y: 442_590_477,
    infinity: false,
};

// Point X
pub const X: Point = Point {
    x: 408_641_257,
    y: 77_546_425,
    infinity: false,
};

// Point R
pub const R: Point = Point {
    x: 169_194_362,
    y: 454_004_252,
    infinity: false,
};

// Point Z
pub const Z: Point = Point {
    x: 51_512_840,
    y: 29_904_980,
    infinity: false,
};

// Point A
pub const A: Point = Point {
    x: 112_404_970,
    y: 181_640_566,
    infinity: false,
};

// Point B
pub const B: Point = Point {
    x: 447_214_710,
    y: 625_708_705,
    infinity: false,
};

// Curve params
pub const CURVE_A: u64 = 2;
pub const _CURVE_B: u64 = 2;

// Random generated number
pub const C: u64 = 574_083_228;
