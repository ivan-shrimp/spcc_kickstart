use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    let staircase_size = input_stairs(input);

    // Our way to climb the staircase is to climb 3 stairs at a time,
    // then if 1 or 2 stairs remain, climb the remaining stairs in one step.
    let steps = {
        // This is integer division, rounding down.
        let triple_stair_steps = staircase_size / 3;
        // Calculate the number of stairs we have walked.
        let stairs_completed = triple_stair_steps * 3;
        // If there are any remaining stairs, add one more step to the result.
        triple_stair_steps + u32::from(stairs_completed != staircase_size)
    };

    writeln!(output, "{steps}").expect("An output error occured");
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
