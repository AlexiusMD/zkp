# ZKP

A small Rust project that implements elliptic-curve point arithmetic over a finite field and verifies a Chaum–Pedersen proof. The executable checks a list of candidate responses against hard-coded statement, commitment, and challenge values, printing whether each candidate verifies.

## Requirements

- Rust and Cargo (edition 2024)

## Run

```sh
cargo run
```

Each output line contains a candidate response followed by `true` if it satisfies both verification equations, or `false` otherwise.

## Tests

Run the included unit tests with:

```sh
cargo test
```

The tests check that the configured points are on the curve and exercise identity, point doubling, scalar multiplication, and the generator order.

## Project layout

- `src/operations.rs` — modular arithmetic helpers
- `src/point.rs` — elliptic-curve point operations
- `src/constants.rs` — field, curve, and demonstration values
- `src/chaum_pedersen.rs` — statement, commitment, and proof verification
- `src/main.rs` — demonstration program and unit tests

This project is intended for learning and experimentation. Originally it was implemented for the `Advanced Cryptography` course in PUCRS' computer science graduate program.
It is not audited or suitable for production cryptography.
