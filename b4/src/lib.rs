mod logic;

use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    for (sticks, diamonds) in input_sticks_diamonds(input) {
        writeln!(
            output,
            "{emeralds}",
            emeralds = logic::emeralds(sticks, diamonds)
        )
        .expect("An output error occured");
    }
}

// Input routine.
fn input_sticks_diamonds(input: impl BufRead) -> impl Iterator<Item = (u32, u32)> {
    let mut reader = read_u32::U32Reader::new(input);

    let test_case_count = reader.read_until_newline();

    // The "sticks" value is terminated by a whitespace character
    // while the "diamonds" value is terminated by a newline character.
    std::iter::repeat_with(move || (reader.read_until_space(), reader.read_until_newline()))
        .take(test_case_count as usize)
}
