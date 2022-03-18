fn main() {
    // Repeat the computation and printing for each pair of input.
    for (price, wallet) in input_price_wallet() {
        println!("{packs}", packs = a5::max_packs(price, wallet));
    }
}

// Input routine.
fn input_price_wallet() -> impl Iterator<Item = (u32, u32)> {
    let reader = read_u32::U32Reader::new();

    let test_case_count = reader.read_until(b'\n');

    std::iter::repeat_with(move || {
        let price = reader.read_until(b' ');
        let wallet = reader.read_until(b'\n');
        (price, wallet)
    })
    .take(test_case_count as usize)
}
