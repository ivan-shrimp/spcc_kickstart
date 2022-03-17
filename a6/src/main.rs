fn main() {
    let mut numbers = input_numbers();

    // Sort the input numbers for `solve`.
    numbers.sort_unstable();

    let result = a6::solve(&numbers);

    // Output routine.
    match result {
        Some(num) => println!("{num}"),
        None => println!("-1"),
    }
}

// Input routine.
fn input_numbers() -> Vec<u32> {
    use std::io::{self, BufRead};

    // A helper reading integers from stdin using a buffer.
    fn read_integer(input: &mut io::StdinLock<'_>, delim: u8, buf: &mut Vec<u8>) -> Option<u32> {
        // Clear the buffer for writing new data into it.
        buf.clear();

        // Read the decimal-encoded number into the buffer.
        let written = input
            .read_until(delim, buf)
            // Panic if any I/O error occured.
            .expect("An error occured when attempting to read from stdin");

        // If nothing was written, EOF is reached and None is returned.
        (written != 0).then(|| {
            // Convert the input into an integer.
            // `parse_partial`, unlike the standard library `parse`,
            // can ignore the delimiter(s) left by `read_until` at the end of the string.
            lexical::parse_partial(buf)
                .expect("Input integer was not between 1 and 10^6")
                .0
        })
    }

    // Obtain a handle to stdin.
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    // Create a buffer to write into.
    let mut buf = Vec::new();

    // Read the number of test cases from stdin.
    let test_case_count = read_integer(&mut stdin_lock, b'\n', &mut buf)
        // Panic if it cannot be obtained.
        .expect("Failed to read the number of test cases from stdin");

    // Create an iterator that reads integers repeatedly.
    let iter_over_numbers =
        std::iter::from_fn(move || read_integer(&mut stdin_lock, b' ', &mut buf))
            // Consider only the first "t" cases.
            // This seems redundant with the `then` check in `read_integer`,
            // but in fact the problem does not specify whether any useless lines will be added to the end,
            // which can lead us to exceed our runtime.
            .take(test_case_count as usize);

    // Collect input numbers.
    let mut numbers = Vec::with_capacity(test_case_count as usize);
    numbers.extend(iter_over_numbers);
    numbers
}
