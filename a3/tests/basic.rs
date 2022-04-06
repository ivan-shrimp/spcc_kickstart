// Required in the `unicode` test.
// This `allow` is here because when rust-analyzer calls clippy,
// it complains if we put this directly above `fn unicode()`.
#![allow(clippy::unicode_not_nfc)]

kickstart_macros::fn_run!(a3);

#[test]
fn samples() {
    run("SPCC", "0\n");
    run("Computer Club", "4\n");
}

#[test]
fn vowels_and_consonants() {
    run("aeiouAEIOU", "10\n");
    run("bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ", "0\n");
}

#[test]
fn short_and_long() {
    // `len(s)` = 1
    run("a", "1\n");
    run("b", "0\n");

    run(&"a".repeat(100), "100\n");
    run(&"b".repeat(100), "0\n");
}

#[test]
#[allow(non_snake_case)] // national security
fn Chinese() {
    // This is equivalent to "\u{5141}".
    // Its second byte is exactly the ASCII character 'A'.
    run("允", "0\n");

    // These strings have more than 100 bytes,
    // but `len(s)` in Python would still return 100.
    run(&"測試".repeat(50), "0\n");
    // Make sure we don't ignore the first or last characters.
    run(&"A測試B".repeat(25), "25\n");
    run(&"B測試A".repeat(25), "25\n");
}

#[test]
fn unicode() {
    // This is equivalent to "\u{0065}\u{0301}".
    // U+0065: 'latin small letter e'
    // U+0301: 'combining acute accent'
    run("é", "0\n");

    // This is equilvalent to "\u{00e9}".
    // U+00e9: 'latin small letter e with acute'
    run("é", "0\n");
}
