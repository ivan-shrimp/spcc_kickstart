// Utilities for searching data biased towards the right.

mod impl_for_slice;

#[cfg(all(test, not(miri)))]
mod benches;
#[cfg(test)]
mod tests;

use std::cmp::Ordering::{self, Greater, Less};

/// An extension trait implementing exponential search from the right for slices.
pub(crate) trait RightExpSearchExt {
    /// The type of element this slice contains.
    type Item;

    /// Searches this sorted slice with a comparator function.
    ///
    /// This is a drop-in replacement for [`slice::binary_search_by`].
    /// See its documentation for examples.
    ///
    /// This function is faster than [`slice::binary_search_by`]
    /// when the comparator returns `Ordering::Less` or `Ordering::Equal` near the end of the slice.
    fn right_exponential_search_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    /// Searches this sorted slice for a given element.
    ///
    /// This is a drop-in replacement for [`slice::binary_search`].
    /// See its documentation for examples.
    ///
    /// This function is faster than [`slice::binary_search`]
    /// when the searched item is likely to be near the end of the slice.
    fn right_exponential_search(&self, item: &Self::Item) -> Result<usize, usize>
    where
        Self::Item: Ord,
    {
        self.right_exponential_search_by(|p| p.cmp(item))
    }

    /// Returns the index of the partition point according to the given predicate
    /// (the index of the first element of the second partition).
    ///
    /// This is a drop-in replacement for [`slice::partition_point`].
    /// See its documentation for examples.
    ///
    /// This function is faster than [`slice::partition_point`]
    /// when the partition point is likely to be near the end of the slice.
    fn right_exponential_partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.right_exponential_search_by(|x| if pred(x) { Less } else { Greater })
            .unwrap_or_else(|i| i)
    }
}
