/// Returns the minimum total distance required to walk to the chat position.
pub fn min_total_distance(classrooms: [u32; 3]) -> u32 {
    use std::cmp;

    let [a, b, c] = classrooms;

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

    max - min
}
