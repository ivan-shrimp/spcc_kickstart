use a6_benchgen::bad_case;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn no_solution(c: &mut Criterion) {
    let bad: Vec<u32> = bad_case().collect();

    c.bench_function("no_solution", |b| b.iter(|| black_box(a6::solve(&bad))));
}

fn with_solution(c: &mut Criterion) {
    // This number has exactly four factors, so when it is added to the bad case,
    // this will be the solution. However, the corresponding smallest multiplier is at least 23 * 29 = 667.
    // As our `a6` implementation starts searching from small multipliers,
    // the search time will not be significantly reduced.
    //
    // The value of this number is `765_049`.
    const MAGIC_NUMBER: u32 = 23 * 29 * 31 * 37;

    let mut bad: Vec<u32> = bad_case().collect();
    // We should not be able to find the magic number within the bad case.
    let insert_magic_point = bad.binary_search(&MAGIC_NUMBER).unwrap_err();
    bad.insert(insert_magic_point, MAGIC_NUMBER);

    c.bench_function("with_solution", |b| {
        b.iter(|| black_box(a6::solve(&bad)));
    });
}

fn with_repeat(c: &mut Criterion) {
    use rand::{distributions::WeightedIndex, prelude::*};

    let random_repeat = WeightedIndex::new([4, 3, 2, 1]).unwrap();
    let mut rng = rand::thread_rng();

    let bad: Vec<u32> = bad_case()
        .into_iter()
        // Repetition probabilities: x1: 40%, x2: 30%, x3: 20%, x4: 10%.
        // These probabilities are defined above with WeightedIndex.
        .flat_map(|num| std::iter::repeat(num).take(random_repeat.sample(&mut rng) + 1))
        .take(1_000_000)
        .collect();

    c.bench_function("with_repeat", |b| {
        b.iter(|| black_box(a6::solve(&bad)));
    });
}

fn extreme_repetition(c: &mut Criterion) {
    let degenerate: Vec<u32> = (2..=999)
        .chain(std::iter::repeat(1009).take(1_000_000 - 998 - 1))
        .chain(std::iter::once(1_000_000))
        .collect();

    c.bench_function("extreme_repetition", |b| {
        b.iter(|| black_box(a6::solve(&degenerate)));
    });
}

criterion_group!(
    benches,
    no_solution,
    with_solution,
    with_repeat,
    extreme_repetition,
);
criterion_main!(benches);
