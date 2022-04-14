/// Returns the number of packs of potato chips one can buy.
#[must_use]
pub fn max_packs(price: u32, wallet: u32) -> u32 {
    // Suppose we convert all the money into coins where each coin is worth $price.
    // Then, any money that cannot be converted into coins cannot be used.
    // The number of coins is exactly the quotient of dividing the money in the wallet by the price.
    let coins = wallet / price;

    // Now, the first bag of chips requires 1 coin, the second requires 2 coins, etc.
    // We want the largest n such that `1 + 2 + 3 + ... + n` <= `coins`.
    // The left hand side is called the nth triangular number and is equal to `n * (n + 1) / 2`.
    // Thus we get `(n * (n + 1) / 2) <= coins`,
    // or `n^2 + n - 2 * coins <= 0`, (^ is exponentiation, not bitwise xor; we don't have bitwise operations here)
    // or `n <= (sqrt(d) - 1) / 2`, (quadratic formula, sqrt is short for square root)
    // where `d` = `8 * coins + 1`.
    // This step does not overflow,
    // because `coins` <= 10^6, so `d` = `8 * coins + 1` <= 8_000_001 < u32::MAX.
    let d = 8 * coins + 1;

    // d <= 8 000 001 < 16 777 216, so there is no loss of precision.
    // (see https://en.wikipedia.org/wiki/Single-precision_floating-point_format#Precision_limitations_on_integer_values)
    #[allow(clippy::cast_precision_loss)]
    let d = d as f32;

    // The result N satisfies
    // `N` <= `(sqrt(d) - 1) / 2` < `N + 1`,
    // or `2 * N + 1` <= `sqrt(d)` < `2 * N + 3`.
    //
    // We should round down `sqrt(d)` to the closest odd number,
    // and avoid rounding up to a higher odd number.
    //
    // Let `x` = `2 * N + 3` be the smallest odd number larger than `sqrt(d)`.
    // `x^2` = `4 * N^2 + 12 * N + 9` = `4 * N * (N + 3) + 9`, with remainder 1 when divided by 8.
    // `d` = `8 * coins + 1` also has remainder 1 when divided by 8.
    // Therefore, `x^2 - d` >= 8.
    // `d` <= 8_000_001, so `sqrt(d)` < 2829, `x` <= 2829, giving `x + sqrt(d)` < 5658.
    // Dividing these gives `x - sqrt(d)` > 0.0014134...,
    // but `f32`s near 2829 can correctly handle tiny differences of around 2^-12 = 0.0002441...
    // so we will not round up to `x`.
    let sqrt_d = d.sqrt();

    // Subtracting 1 and dividing by 2 are trivial and accurate for small floating-point numbers.
    //
    // `d` = `8 * coins + 1` >= 1, so `sqrt(d)` >= 1 and `(sqrt(d) - 1) / 2` is non-negative.
    // Rounding towards zero (truncation) is expected, giving the largest integer below `(sqrt(d) - 1) / 2`.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let packs = ((sqrt_d - 1.) / 2.) as u32;
    packs
}
