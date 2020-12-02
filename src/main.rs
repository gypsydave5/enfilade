mod pin;

use shakmaty::fen;
use shakmaty::{Chess, Position};
use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut fen = String::new();
    stdin.read_line(&mut fen);
    let position = fen::Fen::from_str(&fen)?.position()?;
    let mut square = String::new();
    stdin.read_line(&mut square);
    let square = shakmaty::Square::from_str(&square)?;

    println!("{:?}", pin::is_pin(position, square))
}
