mod logic;

use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    let count = logic::count_vowels(&input_words(input));

    writeln!(output, "{count}").expect("An output error occured");
}

// Input routine.
fn input_words(input: impl BufRead) -> String {
    input
        // Read a line of input.
        .lines()
        .next()
        .expect("No input was provided")
        .expect("An input error occured")
}
