// Tests `max_packs` with low values of `wallet`.
#[test]
fn poor() {
    assert_eq!(a5::max_packs(1, 1), 1);
    assert_eq!(a5::max_packs(2, 1), 0);
    assert_eq!(a5::max_packs(1000, 1), 0);
}

// Tests `max_packs` with high values of `wallet`.
#[test]
fn rich() {
    // 1 + 2 + 3 + ... + 1413 = 998_991, 1 + 2 + 3 + ... + 1414 = 1_000_405
    assert_eq!(a5::max_packs(1, 1_000_000), 1413);
    // 1000 + 2000 + 3000 + ... + 44000 = 990_000, 1000 + 2000 + 3000 + ... + 45000 = 1_035_000
    assert_eq!(a5::max_packs(1000, 1_000_000), 44);
}

use quickcheck::quickcheck;

quickcheck! {
    // Validates that the number of packs calculated by `max_packs` is correct.
    fn check_max_packs(price: u32, wallet: u32) -> bool {
        // Generate a valid `price` (satisfying 1 <= `price` <= 1_000)
        // and a valid `wallet` (satisfying 1 <= `wallet` <= 1_000_000);
        let price = price % 1_000 + 1;
        let wallet = wallet % 1_000_000 + 1;

        // Try to use our `max_packs` function to calculate the maximum number of packs,
        // and check if it's correct below.
        let packs = a5::max_packs(price, wallet);

        // Guard against overflow during multiplication.
        let [price, wallet, packs] = [price, wallet, packs].map(u128::from);

        // Calculate the money required to buy `packs + 1` packs of potato chips.
        let high = (packs + 1) * (packs + 2) / 2 * price;
        // Calculate the money required to buy `packs` packs of potato chips.
        let low = packs * (packs + 1) / 2 * price;

        // Ensure that our wallet can be used to buy `packs` potato chips but not `packs + 1` potato chips.
        (low..high).contains(&wallet)
    }
}
