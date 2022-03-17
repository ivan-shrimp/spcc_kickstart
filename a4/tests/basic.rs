#![cfg(not(miri))] // miri can't run other executables

/// Runs the main executable and writes `input` to stdin (as a string),
/// then ensures that it produces `output` in stdout (as a string).
fn run(input: u32, output: u32) {
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin(format!("{input}"))
        .assert()
        .success()
        .stdout(format!("{output}\n"));
}

#[test]
fn samples() {
    run(15, 5);
    run(100, 34);
}

// Unfortunately, our tests have to assume that our way of
// climbing stairs is correct.
#[test]
fn edge_cases() {
    run(1, 1);
    run(2, 1);
    run(3, 1);
    run(4, 2);

    run(999_997, 333_333);
    run(999_998, 333_333);
    run(999_999, 333_333);
    run(1_000_000, 333_334);
}
