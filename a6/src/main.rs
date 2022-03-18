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
    let reader = read_u32::U32Reader::new();

    // Read the number of test cases from stdin.
    let test_case_count = reader.read_until(b'\n');

    reader.read_line(test_case_count as usize).collect()
}
