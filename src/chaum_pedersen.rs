use crate::point::Point;

pub struct Statement {
    pub g: Point,
    pub x: Point,
    pub r: Point,
    pub z: Point,
}

pub struct Commitment {
    pub a: Point,
    pub b: Point,
}

pub fn verify(statement: &Statement, commitment: &Commitment, challenge: u64, response: u64) -> bool {
    let left_1 = statement.g.scalar_mult(response);

    let right_1 = commitment.a.add(&statement.r.scalar_mult(challenge));

    let left_2 = statement.x.scalar_mult(response);

    let right_2 = commitment.b.add(&statement.z.scalar_mult(challenge));

    left_1 == right_1 && left_2 == right_2
}
