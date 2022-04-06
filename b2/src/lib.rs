use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    // The 7 characters that will not be changed when rotated 180 degrees.
    const LETTERS_ON_SIGN: &[u8] = b"HINOSXZ";

    // Check that every input letter is a letter that can be put on the sign.
    if input_letters(input).all(|letter| LETTERS_ON_SIGN.contains(&letter)) {
        writeln!(output, "YES")
    } else {
        writeln!(output, "NO")
    }
    .expect("An output error occured");
}

// Input routine.
fn input_letters(input: impl BufRead) -> impl Iterator<Item = u8> {
    input
        // Because the input is a string of "all capital letters",
        // we can just iterate through the bytes.
        .bytes()
        .map(|byte| byte.expect("An input error occured"))
        // If it is not a capital letter, we might have reached a newline character; it means we're done.
        .take_while(|&byte| byte.is_ascii_uppercase())
}
