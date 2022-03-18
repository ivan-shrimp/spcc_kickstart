fn main() {
    println!("Hello {name}!", name = input_name());
}

// Input routine.
fn input_name() -> String {
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
}
