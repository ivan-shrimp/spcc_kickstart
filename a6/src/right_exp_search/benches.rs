// The following are benchmarks for `binary_search` and friends in the standard library,
// modified to test our functions.

// the standard library is always correct
#![allow(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate test;

use test::black_box;
use test::Bencher;

use super::RightExpSearchExt;

enum Cache {
    L1,
    L2,
    L3,
}

impl Cache {
    fn size(&self) -> usize {
        match self {
            Cache::L1 => 1000,      // 8kb
            Cache::L2 => 10_000,    // 80kb
            Cache::L3 => 1_000_000, // 8Mb
        }
    }
}

fn right_exponential_search<F>(b: &mut Bencher, cache: Cache, mapper: F)
where
    F: Fn(usize) -> usize,
{
    let size = cache.size();
    let v = (0..size).map(&mapper).collect::<Vec<_>>();
    let mut r = 0usize;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = mapper(r % size);
        black_box(v.right_exponential_search(&i).is_ok());
    });
}

// Different from standard library: this was "worst case" with binary search
// because the rightmost element was searched;
// it is now "best case" with right exponential search.
fn right_exponential_search_best_case(b: &mut Bencher, cache: Cache) {
    let size = cache.size();

    let mut v = vec![0; size];
    let i = 1;
    v[size - 1] = i;
    b.iter(move || {
        black_box(v.right_exponential_search(&i).is_ok());
    });
}

// The bad case for right exponential search is when the leftmost element is searched.
fn right_exponential_search_bad_case(b: &mut Bencher, cache: Cache) {
    let size = cache.size();

    let mut v = vec![1; size];
    let i = 0;
    v[0] = i;
    b.iter(move || {
        black_box(v.right_exponential_search(&i).is_ok());
    });
}

#[bench]
fn right_exponential_search_l1(b: &mut Bencher) {
    right_exponential_search(b, Cache::L1, |i| i * 2);
}

#[bench]
fn right_exponential_search_l2(b: &mut Bencher) {
    right_exponential_search(b, Cache::L2, |i| i * 2);
}

#[bench]
fn right_exponential_search_l3(b: &mut Bencher) {
    right_exponential_search(b, Cache::L3, |i| i * 2);
}

#[bench]
fn right_exponential_search_l1_with_dups(b: &mut Bencher) {
    right_exponential_search(b, Cache::L1, |i| i / 16 * 16);
}

#[bench]
fn right_exponential_search_l2_with_dups(b: &mut Bencher) {
    right_exponential_search(b, Cache::L2, |i| i / 16 * 16);
}

#[bench]
fn right_exponential_search_l3_with_dups(b: &mut Bencher) {
    right_exponential_search(b, Cache::L3, |i| i / 16 * 16);
}

#[bench]
fn right_exponential_search_l1_best_case(b: &mut Bencher) {
    right_exponential_search_best_case(b, Cache::L1);
}

#[bench]
fn right_exponential_search_l2_best_case(b: &mut Bencher) {
    right_exponential_search_best_case(b, Cache::L2);
}

#[bench]
fn right_exponential_search_l3_best_case(b: &mut Bencher) {
    right_exponential_search_best_case(b, Cache::L3);
}

#[bench]
fn right_exponential_search_l1_bad_case(b: &mut Bencher) {
    right_exponential_search_bad_case(b, Cache::L1);
}

#[bench]
fn right_exponential_search_l2_bad_case(b: &mut Bencher) {
    right_exponential_search_bad_case(b, Cache::L2);
}

#[bench]
fn right_exponential_search_l3_bad_case(b: &mut Bencher) {
    right_exponential_search_bad_case(b, Cache::L3);
}
