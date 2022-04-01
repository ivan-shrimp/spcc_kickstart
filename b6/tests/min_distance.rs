use b6::minimum_distance;

macro_rules! classrooms {
    // Convert zeros and ones into `false`s and `true`s respectively.
    ($($bits:literal)*) => {{ [$($bits == 1),*] }};
}

#[test]
// No continuous collection points.
fn discontinuous() {
    // +---+---+---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   3>  2>  1>  *  <1>  *  <1>  *  <1  <2
    assert_eq!(
        minimum_distance(classrooms![0 0 0 1 0 1 0 1 0 0]),
        3 + 2 + 1 + 1 + 1 + 1 + 2,
    );

    // +---+---+---+---+---+---+---+---+---+---+
    // | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 1 | 0 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   2>  1>  *  <1>  *  <1  <2>  1>  *  <1
    assert_eq!(
        minimum_distance(classrooms![0 0 1 0 1 0 0 0 1 0]),
        2 + 1 + 1 + 1 + 2 + 1 + 1,
    );

    // +---+---+---+---+---+---+---+---+---+---+
    // | 0 | 0 | 1 | 0 | 0 | 1 | 0 | 0 | 1 | 0 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   2>  1>  *  <1   1>  *  <1   1>  *  <1
    assert_eq!(
        minimum_distance(classrooms![0 0 1 0 0 1 0 0 1 0]),
        2 + 1 + 1 + 1 + 1 + 1 + 1,
    );
}

#[test]
// Some collection points are continuous.
fn continuous() {
    // +---+---+---+---+---+
    // | 0 | 1 | 1 | 0 | 0 |
    // +---+---+---+---+---+
    //   1>  *   *  <1  <2
    assert_eq!(minimum_distance(classrooms![0 1 1 0 0]), 1 + 1 + 2);

    // +---+---+---+---+---+
    // | 1 | 1 | 0 | 0 | 0 |
    // +---+---+---+---+---+
    //   *   *  <1  <2  <3
    assert_eq!(minimum_distance(classrooms![1 1 0 0 0]), 1 + 2 + 3);

    // +---+---+---+---+---+
    // | 0 | 0 | 0 | 1 | 1 |
    // +---+---+---+---+---+
    //   3>  2>  1>  *   *
    assert_eq!(minimum_distance(classrooms![0 0 0 1 1]), 3 + 2 + 1);

    // +---+---+---+---+---+
    // | 1 | 1 | 1 | 1 | 0 |
    // +---+---+---+---+---+
    //   *   *   *   *  <1
    assert_eq!(minimum_distance(classrooms![1 1 1 1 0]), 1);

    // +---+---+---+---+---+
    // | 0 | 1 | 1 | 1 | 1 |
    // +---+---+---+---+---+
    //   1>  *   *   *   *
    assert_eq!(minimum_distance(classrooms![0 1 1 1 1]), 1);

    // +---+---+---+---+---+
    // | 1 | 1 | 1 | 1 | 1 |
    // +---+---+---+---+---+
    //   *   *   *   *   *
    assert_eq!(minimum_distance(classrooms![1 1 1 1 1]), 0);
}

#[test]
// One or both ends have collection points.
fn sided() {
    // Sided to the left:

    // +---+---+---+---+---+---+---+---+---+---+
    // | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   *  <1  <2  <3  <4  <5  <6  <7  <8  <9
    assert_eq!(
        minimum_distance(classrooms![1 0 0 0 0 0 0 0 0 0]),
        1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9,
    );

    // +---+---+---+---+---+---+---+---+---+---+
    // | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   *  <1  <2  <3   3>  2>  1>  *  <1  <2
    assert_eq!(
        minimum_distance(classrooms![1 0 0 0 0 0 0 1 0 0]),
        1 + 2 + 3 + 3 + 2 + 1 + 1 + 2,
    );

    // Sided to the right:

    // +---+---+---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   9>  8>  7>  6>  5>  4>  3>  2>  1>  *
    assert_eq!(
        minimum_distance(classrooms![0 0 0 0 0 0 0 0 0 1]),
        9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1,
    );

    // +---+---+---+---+---+---+---+---+---+---+
    // | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 1 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   4>  3>  2>  1>  *  <1  <2   2>  1>  *
    assert_eq!(
        minimum_distance(classrooms![0 0 0 0 1 0 0 0 0 1]),
        4 + 3 + 2 + 1 + 1 + 2 + 2 + 1,
    );

    // Both sides:

    // +---+---+---+---+---+---+---+---+---+---+
    // | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   *  <1  <2  <3  <4   4>  3>  2>  1>  *
    assert_eq!(
        minimum_distance(classrooms![1 0 0 0 0 0 0 0 0 1]),
        1 + 2 + 3 + 4 + 4 + 3 + 2 + 1,
    );

    // +---+---+---+---+---+---+---+---+---+---+
    // | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |
    // +---+---+---+---+---+---+---+---+---+---+
    //   *  <1   1>  *  <1  <2  <3>  2>  1>  *
    assert_eq!(
        minimum_distance(classrooms![1 0 0 0 0 0 0 0 0 1]),
        1 + 2 + 3 + 4 + 4 + 3 + 2 + 1,
    );
}

#[test]
fn big() {
    // A '1' followed by 499_999 '0's gives 1 + 2 + 3 + 4 + ... + 499_999.
    // This number overflows a 32-bit integer, likely the reason why an "answer may be large" hint was given.
    let iter = std::iter::once(true).chain(std::iter::repeat(false).take(499_999));
    assert_eq!(minimum_distance(iter), 124_999_750_000);
}
