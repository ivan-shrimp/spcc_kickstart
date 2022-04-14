mod logic;

use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    writeln!(
        output,
        "{min_total_distance}",
        min_total_distance = logic::min_total_distance(input_positions(input))
    )
    .expect("An output error occured");
}

// Input routine.
fn input_positions(input: impl BufRead) -> [u32; 3] {
    let mut reader = read_u32::U32Reader::new(input);
    [
        reader.read_until_space(),   // Alice
        reader.read_until_space(),   // Bob
        reader.read_until_newline(), // Charlie
    ]
}
