use std::cmp::Ordering::{self, Greater, Less};

use super::RightExpSearchExt;

impl<T> RightExpSearchExt for [T] {
    type Item = T;

    fn right_exponential_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        // This method is called "exponential search", explained here:
        // https://en.wikipedia.org/wiki/Exponential_search
        //
        // Example: We have a slice and we wish to look for `14`.
        // Start with a search size of 1, that is, the range from `l` to `r`.
        //
        //                                                    l     r
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //     | 0 | 1 | 2 | 3 | 5 | 7 | 10 | 13 | 15 | 17 | 19 |
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //
        // The value at `l` is larger than 14,
        // so we double the search size to 2,
        // move `l` to the left by 2, and let `r` be the original position of `l`.
        //
        //                                          l         r
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //     | 0 | 1 | 2 | 3 | 5 | 7 | 10 | 13 | 15 | 17 | 19 |
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //
        // Repeat and double the search size to 4.
        //
        //                       l                  r
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //     | 0 | 1 | 2 | 3 | 5 | 7 | 10 | 13 | 15 | 17 | 19 |
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //
        // This time the value at `l` is smaller than 14,
        // so the first stage is done, and we proceed to the "binary search" stage.
        // Find the mid-point `m` of this range.
        //
        //                       l        m         r
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //     | 0 | 1 | 2 | 3 | 5 | 7 | 10 | 13 | 15 | 17 | 19 |
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //
        // The value at `m` is smaller than 14,
        // so we set `l` to the right of `m` and set `m` to the new mid-point.
        //
        //                                    lm    r
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //     | 0 | 1 | 2 | 3 | 5 | 7 | 10 | 13 | 15 | 17 | 19 |
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //
        // The value at `m` is still smaller than 14,
        // so we set `l` to the right of `m` and move `m` to a new mid-point.
        //
        //                                         lmr
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //     | 0 | 1 | 2 | 3 | 5 | 7 | 10 | 13 | 15 | 17 | 19 |
        //     +---+---+---+---+---+---+----+----+----+----+----+
        //
        // Now `l` and `r` are the same, so we decide that the value cannot be found
        // and `l` is returned.
        // `l` satisfies the condition that `slice[..l]` contains values below 14
        // and `slice[l..]` contains values above 14.

        // Range-seeking stage.

        let mut size: usize = 1;
        let mut left = self.len();
        let mut right;

        loop {
            // Set the new position of the left index.
            // If we've reached the leftmost value, enter the binary search stage.
            right = left;
            if left <= size {
                left = 0;
                size = right;
                break;
            }
            left -= size;

            // SAFETY: the call is made safe by the following invariants:
            // - `left < slice.len()`:
            //     - first iteration: `left = slice.len() - 1`.
            //     - `left` is monotonically decreasing through non-underflowing subtraction.
            let cmp = f(unsafe { self.get_unchecked(left) });

            // This is `if`/`else` but not `match` because standard library hackery:
            // https://github.com/rust-lang/rust/blob/803a7593044457cc8436847196752405cf023fb5/library/core/src/slice/mod.rs#L2370
            if cmp == Less {
                // Overshot.
                // Enter the binary search stage.
                break;
            } else if cmp == Greater {
                // Double the search size.
                //
                // `left < slice.len()` (shown above).
                // For non-ZSTs, `slice.len() <= isize::MAX`, explained in:
                // https://github.com/rust-lang/rust/blob/803a7593044457cc8436847196752405cf023fb5/library/core/src/slice/mod.rs#L528
                // Finally, `isize::MAX * 2 < usize::MAX`.
                // Therefore, `left * 2 < usize::MAX`.
                //
                // For ZSTs, we do a runtime check.
                if std::mem::size_of::<T>() == 0 {
                    size = size.saturating_mul(2);
                } else {
                    size *= 2;
                }
            } else {
                // Found the exact value, return its index.
                if left >= self.len() {
                    // SAFETY: same as the `get_unchecked` above.
                    //
                    // The standard library uses `std::intrinsics::assume(mid < slice.len())`
                    // but we are not the standard library,
                    // so we use the stabilized `unreachable_unchecked()`.
                    unsafe { std::hint::unreachable_unchecked() }
                }
                return Ok(left);
            }
        }

        // Binary search stage, extracted from the standard library.

        while left < right {
            let mid = left + size / 2;

            // SAFETY: the call is made safe by the following invariants:
            // - `mid >= 0`
            // - `mid < size`: `mid` is limited by `[left; right)` bound.
            let cmp = f(unsafe { self.get_unchecked(mid) });

            if cmp == Less {
                left = mid + 1;
            } else if cmp == Greater {
                right = mid;
            } else {
                if mid >= self.len() {
                    // SAFETY: same as the `get_unchecked` above.
                    unsafe { std::hint::unreachable_unchecked() }
                }
                return Ok(mid);
            }

            size = right - left;
        }

        Err(left)
    }
}
