use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    // Repeat the computation and printing for each pair of input.
    for (cost, members) in input_cost_members(input) {
        match (cost / members, cost % members) {
            // If there is no remainder, the cost can be evenly split.
            (amount_to_pay, 0) => writeln!(output, "YES {amount_to_pay}"),
            // If there is some remainder, we can subtract that remainder
            // so that the cost can be evenly split.
            (_, remainder) => writeln!(output, "NO {remainder}"),
        }
        .expect("An output erro occured");
    }
}

// Input routine.
fn input_cost_members(input: impl BufRead) -> impl Iterator<Item = (u32, u32)> {
    let mut reader = read_u32::U32Reader::new(input);

    let test_case_count = reader.read_until_newline();

    std::iter::repeat_with(move || {
        let cost = reader.read_until_space();
        let members = reader.read_until_newline();
        (cost, members)
    })
    .take(test_case_count as usize)
}
