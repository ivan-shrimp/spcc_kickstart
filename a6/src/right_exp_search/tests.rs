// The following are tests for `binary_search` and friends in the standard library,
// modified to test our functions.

// the standard library is always correct
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]

use std::cmp::Ordering;

use super::RightExpSearchExt;

#[test]
fn test_search() {
    let b: [i32; 0] = [];
    assert_eq!(b.right_exponential_search(&5), Err(0));

    let b = [4];
    assert_eq!(b.right_exponential_search(&3), Err(0));
    assert_eq!(b.right_exponential_search(&4), Ok(0));
    assert_eq!(b.right_exponential_search(&5), Err(1));

    let b = [1, 2, 4, 6, 8, 9];
    assert_eq!(b.right_exponential_search(&5), Err(3));
    assert_eq!(b.right_exponential_search(&6), Ok(3));
    assert_eq!(b.right_exponential_search(&7), Err(4));
    assert_eq!(b.right_exponential_search(&8), Ok(4));

    let b = [1, 2, 4, 5, 6, 8];
    assert_eq!(b.right_exponential_search(&9), Err(6));

    let b = [1, 2, 4, 6, 7, 8, 9];
    assert_eq!(b.right_exponential_search(&6), Ok(3));
    assert_eq!(b.right_exponential_search(&5), Err(3));
    assert_eq!(b.right_exponential_search(&8), Ok(5));

    let b = [1, 2, 4, 5, 6, 8, 9];
    assert_eq!(b.right_exponential_search(&7), Err(5));
    assert_eq!(b.right_exponential_search(&0), Err(0));

    let b = [1, 3, 3, 3, 7];
    assert_eq!(b.right_exponential_search(&0), Err(0));
    assert_eq!(b.right_exponential_search(&1), Ok(0));
    assert_eq!(b.right_exponential_search(&2), Err(1));
    assert!(match b.right_exponential_search(&3) {
        Ok(1..=3) => true,
        _ => false,
    });
    assert!(match b.right_exponential_search(&3) {
        Ok(1..=3) => true,
        _ => false,
    });
    assert_eq!(b.right_exponential_search(&4), Err(4));
    assert_eq!(b.right_exponential_search(&5), Err(4));
    assert_eq!(b.right_exponential_search(&6), Err(4));
    assert_eq!(b.right_exponential_search(&7), Ok(4));
    assert_eq!(b.right_exponential_search(&8), Err(5));
}

#[test]
fn test_search_by_overflow() {
    let b = [(); usize::MAX];

    assert_eq!(b.right_exponential_search_by(|_| Ordering::Greater), Err(0));
    assert_eq!(
        b.right_exponential_search_by(|_| Ordering::Less),
        Err(usize::MAX)
    );
}

#[test]
fn test_partition_point() {
    let b: [i32; 0] = [];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 5), 0);

    let b = [4];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 3), 0);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 4), 0);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 5), 1);

    let b = [1, 2, 4, 6, 8, 9];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 5), 3);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 6), 3);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 7), 4);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 8), 4);

    let b = [1, 2, 4, 5, 6, 8];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 9), 6);

    let b = [1, 2, 4, 6, 7, 8, 9];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 6), 3);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 5), 3);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 8), 5);

    let b = [1, 2, 4, 5, 6, 8, 9];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 7), 5);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 0), 0);

    let b = [1, 3, 3, 3, 7];
    assert_eq!(b.right_exponential_partition_point(|&x| x < 0), 0);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 1), 0);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 2), 1);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 3), 1);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 4), 4);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 5), 4);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 6), 4);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 7), 4);
    assert_eq!(b.right_exponential_partition_point(|&x| x < 8), 5);
}
