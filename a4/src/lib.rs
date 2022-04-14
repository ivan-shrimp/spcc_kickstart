mod logic;

use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    writeln!(
        output,
        "{steps}",
        steps = logic::min_steps(input_stairs(input))
    )
    .expect("An output error occured");
}

// Input routine.
fn input_stairs(input: impl BufRead) -> u32 {
    input
        // Read a line of input.
        .lines()
        .next()
        .expect("No input was provided")
        .expect("An input error occured")
        // Convert the input into an integer, panicking on failure.
        .parse()
        .expect("Input was not a an integer")
}
