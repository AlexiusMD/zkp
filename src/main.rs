mod operations;
mod point;
mod constants;

use std::io;

fn main() {
    print!("Enter the desired elliptic curve: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to receive input");
}
