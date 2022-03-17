#![cfg(not(miri))] // miri can't run other executables

/// Runs the main executable and writes `input` to stdin,
/// then ensures that it produces `output` in stdout.
fn run(input: &str, output: &'static str) {
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(output);
}

#[test]
fn samples() {
    run("SPCC", "Hello SPCC!\n");
    run("Computer Club", "Hello Computer Club!\n");
}

#[test]
#[allow(non_snake_case)] // national security
fn Chinese() {
    run("聖保羅", "Hello 聖保羅!\n");
}

#[test]
fn newline() {
    // Unix-style newline
    run("SPCC\n", "Hello SPCC!\n");
    // Windows-style newline
    run("SPCC\r\n", "Hello SPCC!\n");
}
