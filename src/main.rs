use std::error::Error;
use std::io;
use std::str::FromStr;

use shakmaty::fen::Fen;
use shakmaty::{Chess, Position, Setup};

use pin::Pinning;

mod pin;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut fen = String::new();
    stdin.read_line(&mut fen)?;
    let fen = fen.trim();
    let setup: Fen = fen.parse()?;
    let position: Chess = setup.position()?;

    let mut mv = String::new();
    stdin.read_line(&mut mv)?;
    let mv = mv.trim();
    let mv = shakmaty::uci::Uci::from_str(mv)?.to_move(&position)?;

    let moved_to = mv.to();
    let new_position = position.play(&mv)?;
    let pins_from_move = new_position.board().pins_involving_square(moved_to);

    for pin in pins_from_move {
        println!("{}", pin);
    }

    Ok(())
}
