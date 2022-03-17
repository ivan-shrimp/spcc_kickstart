fn main() {
    let staircase_size = input_stairs();

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

    // Output routine.
    println!("{steps}");
}

// Input routine.
fn input_stairs() -> u32 {
    use std::io::{self, BufRead};

    // Obtain a handle to stdin.
    io::stdin()
        .lock()
        // Read a line of input.
        .lines()
        .next()
        // Panic if we can't get anything from stdin.
        .expect("No input was provided through stdin")
        // Panic if any I/O error occured.
        .expect("An error occured when attempting to read from stdin")
        // Convert the input into an integer, panicking on failure.
        .parse()
        .expect("Input was not a an integer")
}
