// We are looking for some numbers `(a, b, c)` among the inputs such that `a * b = c`.
// Without loss of generality, assume a <= b (< c).
// Because a * a <= a * b = c <= max, the range of `a` is pretty small.
// So we fix `a` and try to find `b` and `c`.
//
// Example: We start with the inputs sorted from smallest to largest.
// Let `max` be the largest number among all numbers.
//
//     (Variables: max = 100)
//
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 1:
// Begin from the smallest value `a`.
// Ensure that `a` * `a` <= `max`.
//
//     (Variables: a = 2, max = 100)
//
//       **                                 ***
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 2:
// The search range for `b`, denoted `bs` and pronounced "bees",
// is the range of numbers after `a` and no more than `max` / `a`, which is 50.
//
//     (Variables: a = 2, max = 100)
//
//          | <======= `bs` =======> |
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3:
// Find the largest `b` in `bs` such that `c` = `a` * `b` exists in the array.
// In this case, `b` is found to be at most 23 and the corresponding `c` is 46.
//
//     (Variables: a = 2, max = 100)
//
//                           **   **
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Back to Step 1:
// Proceed to the next value of `a`, which is 2 again!
// We've just searched through that, so proceed to the next `a`.
// So `a` is now 3.
//
//     (Variables: current_best_c = 46, a = 3, max = 100)
//
//      done  XX   **
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 2:
// `bs` is, now, the numbers larger than `current_best_c` / `a` (which is 15)
// but no larger than `max` / `a` (which is 33).
//
//     (Variables: current_best_c = 46, a = 3, max = 100)
//
//                        >| bs |<
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3:
// Find the largest `b` in `bs` such that `c` = `a` * `b` exists in the array.
// In this case, there is no such `b`, so we continue.
//
// Step 1 again:
// Proceed to the next possible value of `a`, which is 13.
// But 13 * 13 = 169, which is larger than `max`.
// So we're finally done, and the answer is 46.
//
//     (Variables: max_c_so_far = 46, a = 3, max = 100)
//
//      done  XX  done bye!
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+

// Regarding the computational complexity of this algorithm:
// - Denote `n` as the number of inputs and `max` as the
// - For every (unique, because Step 1) `a` among the inputs such that `a` <= sqrt(max):
//   - In Step 2,
//     we do at most 2 O(log(n)) binary searches to locate the
//     lower and upper bounds of `bs`.
//   - In Step 3,
//     we iterate through `bs` in worst-case O(max / a) time (see code comments below in Step 3).
//
// Now the step 2 search is negligible compared to step 3,
// so the total worst case complexity
// = O(max / 2 + max / 3 + max / 4 + max / 5 + ... + max / sqrt(max))
// = O(max * (1/2 + 1/3 + 1/4 + 1/5 + ... + 1/sqrt(max)))
// = O(max * log(sqrt(max)))
//   [1 + 1/2 + 1/3 + ... + 1/x is approximately ln(x) or "the logarithm of `x` base e",
//   source: <https://en.wikipedia.org/wiki/Harmonic_number>]
// = O(max * log(max)) [log properties].

/// Solves Problem A6. Requires inputs to be sorted and between 2 and 10^6.
#[must_use]
pub fn solve(mut numbers: &[u32]) -> Option<u32> {
    // The maximum value among all numbers.
    // Return if there are no input numbers.
    let &max = numbers.last()?;

    let mut current_best_c: Option<u32> = None;

    // Use a search table to improve search performance.
    let numbers_table = numbers
        .iter()
        // For `numbers_table[max]` to be valid, create a table with `max` + 1 entries.
        // This uses not O(n) but O(max) memory, so if the input is [2, 3, 1_000_000],
        // we will still allocate 1 MB of memory.
        // But we don't have a memory limit and 1 MB is negligible compared to firing up Python or Scratch,
        // so it doesn't matter.
        //
        // Theoretically we can use HashSet (O(n) memory) or BitVec (1/8 of current allocation),
        // but searching in them involves a rather significant constant time factor.
        .fold(vec![false; (max + 1) as usize], |mut table, &num| {
            table[num as usize] = true;
            table
        });

    // Remember the value of `a` in the previous iteration.
    // `a` cannot be 0, but the "correct" solution `Option<NonZeroU32>` is quite clumsy,
    // so we just use 0 as a sentinel.
    let mut previous_a = 0;

    loop {
        // Step 1.

        // Extract `a` from the data.
        let (&a, tail) = numbers
            .split_first()
            // For the first iteration, if there are no inputs, we should have returned when getting `max`.
            // For the other iterations, if there are no more numbers,
            // we should have returned below, because `max` * `max` > `max`.
            .unwrap_or_else(|| unreachable!());

        // Remove `a` from `numbers`.
        numbers = tail;

        // Check that we are not repeating our work.
        if std::mem::replace(&mut previous_a, a) == a {
            continue;
        }

        // Check if we are done.
        // Guard against overflow.
        // (Suppose the inputs are [2, 3, 92682, 92683, 92684].
        //  On the third iteration we would realize 92682u32 * 92682u32 = 18532u32 < 92684u32,
        //  so we would continue if without the guard.)
        if u64::from(a) * u64::from(a) > u64::from(max) {
            break;
        }

        // Step 2.

        // Set the search range for `b`.
        let bs = {
            // Find the upper bound for `b`.
            let b_max = max / a;
            // Extract the part of `numbers` whose values are no more than the upper bound.
            let bs_end = numbers.partition_point(|&b| b <= b_max);

            match current_best_c {
                Some(best_c) => {
                    // If we have a current best `c`, find the lower bound for `b`.
                    let b_min = best_c / a;
                    // Extract the part of `numbers` whose values are strictly larger than the lower bound.
                    // (For `b`s equal to the lower bound, `a` * `b` will not be larger than the current best `c`.)
                    let bs_begin = numbers.partition_point(|&b| b <= b_min);
                    &numbers[bs_begin..bs_end]
                }
                None => &numbers[..bs_end],
            }
        };

        // Step 3.

        // There are two ways we can iterate through `bs` from largest to smallest:
        // 1. Iterating over the `bs` slice directly.
        // 2. Iterating over the `true` entries in `numbers_table` that correspond to `bs`.
        //
        // The first method requires O(`bs.len()`) time,
        // while the second method requires O(last value in bs - first value in bs) time.
        // In particular, the first one is at most around `n` while the second is at most around `max` / `a`.
        //
        // We dynamically select between these two methods to improve performance.
        // (See the `extreme_repetition` benchmark for a case that would be
        // extremely slow if only method 1 is used.)

        let method_1 = || {
            bs.iter()
                // Iterate from largest to smallest.
                .rev()
                // Calculate `c`.
                .map(move |&b| a * b)
                // Look for a `c` that is within the set of numbers.
                .find(|&c| numbers_table[c as usize])
        };

        let method_2 = |first: u32, last: u32| {
            (first..=last)
                // Iterate from largest to smallest.
                .rev()
                // Only consider those between `first` and `last` that actually exist in `bs`.
                .filter(|&b| numbers_table[b as usize])
                // Calculate `c`.
                .map(move |b| a * b)
                // Look for a `c` that is within the set of numbers.
                .find(|&c| numbers_table[c as usize])
        };

        // We know `bs.len()` is not more than 10^6, so converting into a `u32` does not truncate.
        #[allow(clippy::cast_possible_truncation)]
        let new_c = match bs {
            // Heuristics for checking if Method 2 would be faster.
            &[first, .., last]
                if {
                    // Make it slightly more unlikely for Method 2 to be chosen,
                    // because it's generally slower when the data is uniformly distibuted.
                    const METHOD_2_THRESHOLD_RATIO: u32 = 2;
                    (last - first) * METHOD_2_THRESHOLD_RATIO < (bs.len() as u32)
                } =>
            {
                method_2(first, last)
            }
            _ => method_1(),
        };

        // Update the records.
        // In Step 2 we have already ensured that if any new `c` is found,
        // it must be better than the previous one(s).
        if let Some(c) = new_c {
            current_best_c = Some(c);
        }
    }

    current_best_c
}
