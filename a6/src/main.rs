fn main() {
    let mut numbers = input_numbers();

    // Sort the input numbers for `solve`.
    numbers.sort_unstable();

    let result = a6::solve(&numbers);

    match result {
        Some(num) => println!("{num}"),
        None => println!("-1"),
    }
}

// Input routine.
fn input_numbers() -> Vec<u32> {
    let mut reader = read_u32::U32Reader::with_stdin();

    // Read the number of test cases from stdin.
    let test_case_count = reader.read_until_newline() as usize;

    let mut numbers = Vec::with_capacity(test_case_count);
    for _ in 0..test_case_count - 1 {
        numbers.push(reader.read_until_space());
    }
    numbers.push(reader.read_until_newline());

    numbers
}
