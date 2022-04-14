kickstart_macros::fn_run!(b6);

// More tests on the logic in `min_distance.rs`.
// These are mainly I/O tests, to test our custom input iterator.

#[test]
fn sample() {
    run(
        "2\n\
        3\n111\n\
        5\n10010",
        "0\n3\n",
    );
}

#[test]
fn newlines() {
    run(
        "5\n\
        10\n1111111111\n\
        10\n1010101010\n\
        10\n0101010101\n\
        2\n10\n\
        1\n1\n",
        "0\n5\n5\n1\n0\n",
    );
    run(
        "5\r\n\
        10\r\n1111111111\r\n\
        10\r\n1010101010\r\n\
        10\r\n0101010101\r\n\
        2\r\n10\r\n\
        1\r\n1\r\n",
        "0\n5\n5\n1\n0\n",
    );
}
