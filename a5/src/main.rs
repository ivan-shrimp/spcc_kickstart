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
    let reader = read_u32::U32Reader::new();

    let test_case_count = reader.read_until(b'\n');

    std::iter::repeat_with(move || (reader.read_until(b' '), reader.read_until(b'\n')))
        .take(test_case_count as usize)
}
