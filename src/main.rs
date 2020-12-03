use std::error::Error;
use std::io;
use std::str::FromStr;

use shakmaty::fen;

mod pin;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut fen = String::new();
    stdin.read_line(&mut fen)?;
    let fen = fen.trim();
    println!("{:?}", fen);

    let position = fen::Fen::from_str(&fen)?.position()?;
    let mut square = String::new();
    stdin.read_line(&mut square)?;
    let square = square.trim();
    let square = shakmaty::Square::from_str(&square)?;

    println!("{:?}", pin::is_pin(&position, square));
    Ok(())
}
