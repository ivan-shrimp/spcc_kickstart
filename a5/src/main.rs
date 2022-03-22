fn main() {
    // Repeat the computation and printing for each pair of input.
    for (price, wallet) in input_price_wallet() {
        println!("{packs}", packs = a5::max_packs(price, wallet));
    }
}

// Input routine.
fn input_price_wallet() -> impl Iterator<Item = (u32, u32)> {
    let mut reader = read_u32::U32Reader::with_stdin();

    let test_case_count = reader.read_until_newline();

    std::iter::repeat_with(move || {
        let price = reader.read_until_space();
        let wallet = reader.read_until_newline();
        (price, wallet)
    })
    .take(test_case_count as usize)
}
