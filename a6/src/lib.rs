#![cfg_attr(test, feature(test))] // for `right_exp_search`'s benchmarks
#![allow(clippy::redundant_pub_crate)]

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
// The search range for `c`, denoted `cs` and pronounced "seas",
// is the range of numbers after `a `.
// (The theoretical lower bound for `cs` is `a` * `a`.
//  However, because everything in `bs` is no less than `a`,
//  we will meet the lower bound of `bs` first when searching.)
//
//     (Variables: a = 2, max = 100)
//
//          | <======= `bs` =======> |
//          | <============= `cs` ============> |
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3A:
// Let `b` be the largest number in `bs`, which is 46.
// Look for `a` * `b` = 92 in `cs`.
// As it is not found, reduce the upper bound of `cs` to numbers less than 92.
// (Because the numbers around the product (92) is usually near the right of `cs`,
//  a specialized searching method, in `/right_exp_search`,
//  is designed for this usecase.)
//
//     (Variables: a = 2, max = 100)
//
//          | <======= `bs` =======> |
//          | <========== `cs` =========> |
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3B:
// Let `c` be the largest number in `cs`, which is 91.
// Reduce the upper bound of `bs` to numbers no more than `c` / `a`, or 91 / 2 = 45.
// (Again, the new right boundary of `bs` is usually close to its original right boundary,
//  so the specialized searching method is used.)
//
//     (Variables: a = 2, max = 100)
//
//          | <===== `bs` ====> |
//          | <========== `cs` =========> |
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3A again:
// Let `b` be the largest number in `bs`, which is 23.
// Look for `a` * `b` = 46 in `cs`.
// As it is found, mark down 46 as a possible value of `c`.
// (We do not need to continue searching with this `a` with smaller `b`s,
//  because that would yield smaller `c`s.)
//
//     (Variables: a = 2, max = 100)
//
//          | <===== `bs` ====> |
//          | <========== `cs` =========> |
//       **                  **   **
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 4:
// A new `c` is found to be 46.
// Therefore, we can reduce the next `cs` to that above 46.
//
//     (Variables: max_c_so_far = 46, a = 2, max = 100)
//
//                                  >|reduced_cs|<
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Back to Step 1:
// Proceed to the next value of `a`, which is 2 again!
// We've just searched through that, so proceed to the next `a`.
// So `a` is now 3.
//
//     (Variables: max_c_so_far = 46, a = 3, max = 100)
//
//      done  XX   **               >|reduced_cs|<
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 2:
// `bs` is, again, the numbers after `a` but no larger than `max` / `a`, which is 33.
// However, this time we already have a better, reduced `cs`.
//
//     (Variables: max_c_so_far = 46, a = 3, max = 100)
//
//                   >|    bs   |<
//                                  >|    cs    |<
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3A:
// Let `b` be the largest number in `bs`, which is 23.
// Look for `a` * `b` = 69 in `cs`.
// As it is not found, reduce the upper bound of `cs` to numbers less than 92.
//
//     (Variables: max_c_so_far = 46, a = 3, max = 100)
//
//                   >|    bs   |<
//                                  !!!
//     +----+----+----+----+----+----+----+-----+
//     |  2 |  2 |  3 | 13 | 23 | 46 | 91 | 100 |
//     +----+----+----+----+----+----+----+-----+
//
// Step 3B:
// Let `c` be the largest number in `cs`... which is already empty. So we bail out.
//
// Step 4:
// No new `c` is found, no modifications to the next `cs`.
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

// Utilities for searching for data biased towwards the right.
// Improves stage 3 performance.
mod right_exp_search;

/// Solves Problem A6. Requires inputs to be sorted and between 2 and 10^6.
#[allow(clippy::items_after_statements)] // put step 3 after steps 1 and 2
#[must_use]
pub fn solve(mut numbers: &[u32]) -> Option<u32> {
    // The maximum value among all numbers.
    // Return if there are no input numbers.
    let &max = numbers.last()?;

    // If any `c` has been found, store the value of `c` and the range of numbers above `c`.
    let mut best_c_with_reduced_cs: Option<(u32, &[u32])> = None;

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
            .expect("internal error: this is a bug!");

        // Remove `a` from `numbers`.
        numbers = tail;

        // Check that we are not repeating our work.
        if a == previous_a {
            continue;
        }

        // Record down this `a` for the next iteration.
        previous_a = a;

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
            // Find the maximum possible value for `b`.
            let b_max = max / a;

            // Extract the part of `numbers` whose values are no more than `b`.
            let bs_end = numbers.partition_point(|&b| b <= b_max);
            &numbers[..bs_end]
        };

        // Set the search range for `c`.
        let cs = match best_c_with_reduced_cs {
            // If there is a previous `c`,
            // and it reduces the search range to something smaller than the complete range,
            // use the reduced search range.
            Some((_c, reduced_cs)) if reduced_cs.len() < numbers.len() => reduced_cs,
            // Otherwise, use the complete search range.
            _ => numbers,
        };

        // Step 3.

        let find_c_result = find_c(a, bs, cs);

        /// If a `c` can be found, returns `(c, position_of_c_in_cs)`.
        fn find_c(a: u32, mut bs: &[u32], mut cs: &[u32]) -> Option<(u32, usize)> {
            use self::right_exp_search::RightExpSearchExt;

            loop {
                // Step 3A.

                // Get the next largest `b`.
                // If there are no more possible `b`s, return nothing.
                let &b = bs.last()?;

                // Get the product we wish to look for in `cs`.
                // Does not overflow because `a` * `a` <= `max` <= 10^6, so `a` <= 10^3.
                // `b` <= `max` <= 10^6, so `a` * `b` <= 10^9 < u32::MAX.
                let product = a * b;

                match cs.right_exponential_search(&product) {
                    // As we always reduce `cs` from the right,
                    // this position is the same as the position of `c` relative to the initial `cs`.
                    Ok(position) => return Some((product, position)),
                    // Can't find the product, reduce the range of `c`s.
                    Err(position) => cs = &cs[..position],
                }

                // Step 3B.

                // Get the next largest `c`.
                // If there are no more possible `c`s, return nothing.
                let &c = cs.last()?;

                // Get the maximum value of `b` in the next iteration.
                let b_max = c / a;

                // Reduce the range for `b`s.
                let position = bs.right_exponential_partition_point(|&b| b <= b_max);
                bs = &bs[..position];
            }
        }

        // Step 4.

        // If an improved `c` is found, save this result
        // and reduce the future search range to what is to the right of this `c`.
        if let Some((c, position_of_c_in_cs)) = find_c_result {
            best_c_with_reduced_cs = Some((c, &cs[position_of_c_in_cs + 1..]));
        }
    }

    // Extract the best `c` we have found.
    best_c_with_reduced_cs.map(|(c, _cs)| c)
}
