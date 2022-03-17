// miri can't run other executables
#![cfg(not(miri))]
// Required in the `unicode` test.
// This `allow` is here because when rust-analyzer calls clippy,
// it complains if we put this directly above `fn unicode()`.
#![allow(clippy::unicode_not_nfc)]

/// Runs the main executable and writes `input` to stdin,
/// then ensures that it produces `output` in stdout (as a string).
fn run(input: &str, output: u32) {
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(format!("{output}\n"));
}

#[test]
fn samples() {
    run("SPCC", 0);
    run("Computer Club", 4);
}

#[test]
fn vowels_and_consonants() {
    run("aeiouAEIOU", 10);
    run("bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ", 0);
}

#[test]
fn short_and_long() {
    // `len(s)` = 1
    run("a", 1);
    run("b", 0);

    run(&"a".repeat(100), 100);
    run(&"b".repeat(100), 0);
}

#[test]
#[allow(non_snake_case)] // national security
fn Chinese() {
    // This is equivalent to "\u{5141}".
    // Its second byte is exactly the ASCII character 'A'.
    run("允", 0);

    // these strings have more than 100 bytes,
    // but `len(s)` in Python would still return 100.
    run(&"測試".repeat(50), 0);
    // make sure we don't ignore the first or last characters
    run(&"A測試B".repeat(25), 25);
    run(&"B測試A".repeat(25), 25);
}

#[test]
fn unicode() {
    // This is equivalent to "\u{0065}\u{0301}".
    // U+0065: 'latin small letter e'
    // U+0301: 'combining acute accent'
    run("é", 0);

    // This is equilvalent to "\u{00e9}".
    // U+00e9: 'latin small letter e with acute'
    run("é", 0);
}
