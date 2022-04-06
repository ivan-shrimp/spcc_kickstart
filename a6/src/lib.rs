use std::io::prelude::*;

mod logic;

// public for direct testing
pub use logic::solve;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    let mut numbers = input_numbers(input);

    // Sort the input numbers for `solve`.
    numbers.sort_unstable();

    let result = solve(&numbers);

    match result {
        Some(num) => writeln!(output, "{num}"),
        None => writeln!(output, "-1"),
    }
    .expect("An output error occured");
}

// Input routine.
fn input_numbers(input: impl BufRead) -> Vec<u32> {
    let mut reader = read_u32::U32Reader::new(input);

    // Read the number of test cases.
    let test_case_count = reader.read_until_newline() as usize;

    let mut numbers = Vec::with_capacity(test_case_count);
    for _ in 0..test_case_count - 1 {
        numbers.push(reader.read_until_space());
    }
    numbers.push(reader.read_until_newline());

    numbers
}
