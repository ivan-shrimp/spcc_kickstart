mod logic;

use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    // Note that if the input is "37.49999999..." with sufficiently many 9s,
    // it will be parsed to 37.5 and this will return "NO".
    // We assume this case doesn't exist, as the description
    // "1 floating point number" implies the input should be
    // parsable into a C/C++ `float` / `double`
    // (corresponding to `f32` / `f64`).
    if logic::is_normal_temperature(input_temp(input)) {
        writeln!(output, "YES")
    } else {
        writeln!(output, "NO")
    }
    .expect("An output error occured");
}

// Input routine.
fn input_temp(input: impl BufRead) -> f64 {
    input
        // Read a line of input.
        .lines()
        .next()
        .expect("No input was provided")
        .expect("An input error occured")
        // Convert the input string to a floating point number.
        //
        // While one might want to test digit by digit (first digit should be 3, second should be 6 or 7, ...),
        // there are some weird cases like `3.65e0001`, which is a valid floating point literal
        // and the `parse` function can handle that.
        .parse()
        .expect("Input was not a floating-point literal")
}
