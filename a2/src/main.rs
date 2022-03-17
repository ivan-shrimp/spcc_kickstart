fn main() {
    // Note that if the input is "37.49999999..." with sufficiently many 9s,
    // it will be parsed to 37.5 and this will return "NO".
    // We assume this case doesn't exist, as the description
    // "1 floating point number" implies the input should be
    // parsable into a C/C++ `float` / `double`
    // (corresponding to `f32` / `f64`).
    let output = if (36.0..37.5).contains(&input_temp()) {
        "YES"
    } else {
        "NO"
    };

    // Output routine.
    println!("{output}");
}

// Input routine.
fn input_temp() -> f64 {
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
        // Convert the input string to a floating point number, panicking on failure.
        //
        // While one might want to test digit by digit (first digit should be 3, second should be 6 or 7, ...),
        // there are some weird cases like `3.65e0001`, which is a valid floating point literal
        // and the `parse` function can handle that.
        .parse()
        .expect("Input was not a floating-point literal")
}
