use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn invalid_fen() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;

    cmd.arg("foobar");
    cmd.assert()
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
