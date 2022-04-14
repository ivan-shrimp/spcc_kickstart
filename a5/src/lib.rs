use std::io::prelude::*;

mod logic;

// public for direct testing
pub use logic::max_packs;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    // Repeat the computation and printing for each pair of input.
    for (price, wallet) in input_price_wallet(input) {
        writeln!(output, "{packs}", packs = max_packs(price, wallet))
            .expect("An output error occured");
    }
}

// Input routine.
fn input_price_wallet(input: impl BufRead) -> impl Iterator<Item = (u32, u32)> {
    let mut reader = read_u32::U32Reader::new(input);

    let test_case_count = reader.read_until_newline();

    std::iter::repeat_with(move || {
        let price = reader.read_until_space();
        let wallet = reader.read_until_newline();
        (price, wallet)
    })
    .take(test_case_count as usize)
}
