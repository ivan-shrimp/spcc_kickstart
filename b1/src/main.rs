fn main() {
    // Repeat the computation and printing for each pair of input.
    for (cost, members) in input_cost_members() {
        match (cost / members, cost % members) {
            // If there is no remainder, the cost can be evenly split.
            (amount_to_pay, 0) => println!("YES {amount_to_pay}"),
            // If there is some remainder, we can subtract that remainder
            // so that the cost can be evenly split.
            (_, remainder) => println!("NO {remainder}"),
        }
    }
}

// Input routine.
fn input_cost_members() -> impl Iterator<Item = (u32, u32)> {
    let reader = read_u32::U32Reader::new();

    let test_case_count = reader.read_until(b'\n');

    std::iter::repeat_with(move || {
        let cost = reader.read_until(b' ');
        let members = reader.read_until(b'\n');
        (cost, members)
    })
    .take(test_case_count as usize)
}
