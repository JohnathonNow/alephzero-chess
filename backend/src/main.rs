use num_bigint::BigInt;
use std::str::FromStr;

mod piece;
mod board;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = BigInt::from_str("1234543234343434")?;
    println!("{:?}", x);
    let b = board::Board::new();
    Ok(())
}
