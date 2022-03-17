fn main() {
    // Repeat the computation and printing for each pair of input.
    for (price, wallet) in input_price_wallet() {
        let packs = a5::max_packs(price, wallet);

        // Output routine.
        println!("{packs}");
    }
}

// Input routine.

/// Returns an iterator over the input pairs of `(x, w)`,
/// where `x` is the price of the potato chips
/// and `w` is the amount of money in the wallet.
fn input_price_wallet() -> impl Iterator<Item = (u32, u32)> {
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

    // Create a buffer to write into.
    let mut buf = Vec::new();

    // Read the number of test cases from stdin.
    let test_case_count = read_integer(&mut stdin.lock(), b'\n', &mut buf)
        // Panic if it cannot be obtained.
        .expect("Failed to read the number of test cases from stdin");

    // Create an iterator that reads pairs of integers repeatedly.
    std::iter::from_fn(move || {
        // Read the price of this test case.
        // If it cannot be obtained, the end is reached.
        let price = read_integer(&mut stdin.lock(), b' ', &mut buf)?;
        // Read the money in the wallet in this test case.
        // Panic if we can obtain `price` but not `wallet`.
        let wallet = read_integer(&mut stdin.lock(), b'\n', &mut buf)
            .expect("Failed to read the amount of money in Teddy's wallet after reading the price of potato chips");
        Some((price, wallet))
    })
    // Consider only the expected number of cases.
    // This seems redundant with the check in obtaining "price",
    // but in fact the problem does not specify whether any useless lines will be added to the end,
    // which can lead us to exceed our runtime.
    .take(test_case_count as usize)
}
