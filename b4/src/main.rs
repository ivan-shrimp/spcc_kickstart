fn main() {
    for (sticks, diamonds) in input_sticks_diamonds() {
        // To solve this problem, we define two more currencies.
        //
        // Let every pair of one stick and one diamond be a "pair" token.
        // Let every remaining stick/diamond be an "single" token.
        // Notice that "single" tokens now cannot form any emeralds on their own.
        //
        // Our logic is rewritten as follows:
        // - 1 "single" token and 1 "pair" token give 1 emerald.
        // - 3 "pair" tokens give 2 emeralds.
        // - 2 "pair" tokens give 1 emerald.

        let (pair, single) = if sticks > diamonds {
            (diamonds, sticks - diamonds)
        } else {
            (sticks, diamonds - sticks)
        };

        let emeralds = if single >= pair {
            // If "pair" tokens are scarce and we have enough "single" tokens,
            // then the number of emeralds is the number of "pair" tokens.
            pair
        } else {
            // If "single" tokens are scarce,
            // we greedily convert all "single" tokens into emeralds,
            // and deal with the remaining "pair" tokens.
            //
            // As for the remaining "pair" tokens,
            // we can have the following conversion table:
            //
            // "pair" tokens | Emeralds
            // ------------- | --------
            //             0 | 0
            //             1 | 0
            //             2 | 1
            //             3 | 2
            //             4 | 2
            //             5 | 3
            //             6 | 4
            //             7 | 4
            //              ...
            //
            // which can be cleanly expressed as `emeralds` = `pair` * 2 / 3.
            single + (pair - single) * 2 / 3
        };

        println!("{emeralds}");
    }
}

// Input routine.
fn input_sticks_diamonds() -> impl Iterator<Item = (u32, u32)> {
    let reader = read_u32::U32Reader::new();

    let test_case_count = reader.read_until(b'\n');

    // The "sticks" value is terminated by a whitespace character
    // while the "diamonds" value is terminated by a newline character.
    std::iter::repeat_with(move || (reader.read_until(b' '), reader.read_until(b'\n')))
        .take(test_case_count as usize)
}
