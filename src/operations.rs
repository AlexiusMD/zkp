pub fn mod_add(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

pub fn mod_sub(a: u64, b: u64, p: u64) -> u64 {
    (((a as u128 + p as u128) - b as u128) % p as u128) as u64
}

pub fn mod_mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

pub fn mod_pow(mut base: u64, mut exp: u64, p: u64) -> u64 {
    let mut result = 1u64;

    base %= p;

    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, p);
        }

        base = mod_mul(base, base, p);
        exp >>= 1;
    }

    result
}

pub fn mod_inv(a: u64, p: u64) -> u64 {
    assert!(a != 0, "zero não possui inverso modular");

    mod_pow(a, p - 2, p)
}

pub fn mod_div(a: u64, b: u64, p: u64) -> u64 {
    assert!(b != 0, "divisão por zero");

    mod_mul(a, mod_inv(b, p), p)
}

pub fn mod_neg(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
