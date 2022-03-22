//! A generator of bad case data that most A6 solutions will likely take a long time to solve.

/// Maximum input value for Problem A6.
pub const A6_MAX: u32 = 1_000_000;
const SQRT_A6_MAX: u32 = 1_000;

/// Minimum input value for Problem A6.
pub const A6_MIN: u32 = 2;

/// Returns a set of bad case data for A6, which contains:
/// - All numbers with 2, 3, 7, 8, 12, 13, 17, 18, 22, 23, 27 or 28 total prime factors; and
/// - Primes above `A6_MAX / 4`. (4 is the smallest number in this set)
pub fn bad_case() -> impl Iterator<Item = u32> {
    const WANTED_NUMBER_OF_PRIME_FACTORS: [u32; 12] = [2, 3, 7, 8, 12, 13, 17, 18, 22, 23, 27, 28];

    let count_prime_factors = prime_counter();

    (A6_MIN..=A6_MAX).filter(move |&n| {
        let prime_factors = count_prime_factors(n);
        WANTED_NUMBER_OF_PRIME_FACTORS.contains(&prime_factors)
            || (n > A6_MAX / 4 && prime_factors == 1)
    })
}

/// Creates a function that counts the number of prime factors of any number below `A6_MAX`.
/// e.g.: `prime_counter()(6)` is 2, `prime_counter()(9)` is also 2.
/// This is known as the "big omega" variant of the "prime omega function", see
/// <https://en.wikipedia.org/wiki/Prime_omega_function>.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn prime_counter() -> impl Fn(u32) -> u32 {
    // A sieve to `sqrt(N)` can factorize numbers up to `N`.
    let sieve = primal::Sieve::new(SQRT_A6_MAX as usize);

    move |num: u32| {
        sieve
            // Factorize the number.
            // Does not truncate as we're not running on 16-bit targets. We don't.
            .factor(num as usize)
            // The input, which should not be more than `A6_MAX`, can always be factorized by `sieve`.
            .unwrap()
            .into_iter()
            // Sum up the exponents of each prime factor.
            // Does not truncate because a number can't have more prime factors than itself.
            .map(|(_, exp)| exp as u32)
            .sum()
    }
}
