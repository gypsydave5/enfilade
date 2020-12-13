use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn invalid_fen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;

    let output = cmd.with_stdin().buffer("nonsense").output()?;
    output
        .assert()
        .failure()
        .stderr(predicate::str::contains("InvalidBoard"));

    Ok(())
}

#[test]
fn invalid_position() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    let no_kings_fen = "8/8/8/8/8/8/1r2P3/8 w - - 0 1";

    let output = cmd.with_stdin().buffer(no_kings_fen).output()?;

    output
        .assert()
        .failure()
        .stderr(predicate::str::contains("MISSING_KING"));

    Ok(())
}

#[test]
fn classic_Bb5_pin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    let position = "r1bqkbnr/ppp2ppp/2np4/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 2 4";
    let mv = "f1b5";
    let output = cmd
        .with_stdin()
        .buffer([position, mv].join("\n"))
        .output()?;

    output
        .assert()
        .success()
        .stdout("The White Bishop on b5 now pins the Black Knight on c6 to the Black King on e8\n");

    Ok(())
}

#[test]
fn classic_Bb5_protected() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    let position = "rnbqkbnr/ppp2ppp/3p4/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 1 3";
    let mv = "b8c6";
    let output = cmd
        .with_stdin()
        .buffer([position, mv].join("\n"))
        .output()?;

    output
        .assert()
        .success()
        .stdout("The White Bishop on b5 now pins the Black Knight on c6 to the Black King on e8\n");

    Ok(())
}
