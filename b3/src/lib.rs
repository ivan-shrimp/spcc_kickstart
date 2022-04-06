use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
    use std::cmp;

    let [a, b, c] = input_positions(input);

    // Without loss of generality, let the three classrooms be numbered `x`, `y` and `z`
    // with `x` <= `y` <= `z`. Let `c` be the optimal chat position.
    //
    // Note that `x` <= `c` <= `z`,
    // because if `c` < `x`, then `x` is a better chat position as it is closer to all `x`, `y` and `z`;
    // similarly for `c` > `z`.
    //
    // We want to minimize |`x` - `c`| + |`y` - `c`| + |`z` - `c`| (|a - b| means "the distance between a and b")..
    // The first term must be `c` - `x` and the last term must be `z` - `c` because `x` <= `c` <= `z`.
    // Then we can simplify the sum into `z` - `x` + |`y` - `c`|.
    // The first two terms are constants; and to minimize the third term, we set `c` = `y`.
    // Therefore, `y` (the middle classroom) is the best chat position,
    // and the corresponding total distance is just `z` - `x`.
    //
    // Which means, all we need to do is subtract the largest input from the smallest input.

    let min = cmp::min(cmp::min(a, b), c);
    let max = cmp::max(cmp::max(a, b), c);

    writeln!(output, "{}", max - min).expect("An output error occured");
}

// Input routine.
fn input_positions(input: impl BufRead) -> [u32; 3] {
    let mut reader = read_u32::U32Reader::new(input);
    [
        reader.read_until_space(),   // Alice
        reader.read_until_space(),   // Bob
        reader.read_until_newline(), // Charlie
    ]
}
