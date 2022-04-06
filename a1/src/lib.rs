use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    writeln!(output, "Hello {name}!", name = input_name(input)).expect("An output error occured");
}

fn input_name(input: impl BufRead) -> String {
    input
        // Read a line of input.
        .lines()
        .next()
        .expect("No input was provided")
        .expect("An input error occured")
}
