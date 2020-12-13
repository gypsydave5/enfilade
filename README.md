# Enfilade - a Pin Detector

Enfilade extends [Shakmaty](https://docs.rs/shakmaty/0.17.0/shakmaty/), a Rust Chess library, with methods to analyse pins on a chess board. It exposes these through a command line interface which accepts a [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) and a move in [long algebraic notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)#Long_algebraic_notation), and returns a text description of any pins involving the moved piece, either as the attacker, the target, or the pinned piece.

## Requirements

[Rust](https://www.rust-lang.org/); any version that conforms to the Rust 2018 standard with the concomitant tooling, notably `cargo`.

## Build

`cargo build`

## Run

The package can be run and built with the command.

`cargo run`

## Tests

`cargo test`

## Structure

Loosely, the integration tests are in the `tests` folder at the top level, the `main.rs` file is the entry point for the command line tool, and the `pin` directory contains the library code to make it all work.