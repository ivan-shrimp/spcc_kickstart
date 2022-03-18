#![cfg(not(miri))] // miri can't run other executables

test_run_bin::initialize!();

#[test]
fn sample() {
    run("2\n1 3\n1 11", "2\n4\n");
}

#[test]
fn long() {
    // Build the input string.
    let mut input = String::from("100\n");
    for wallet in 1..=100 {
        use std::fmt::Write;
        // This will leave a trailing newline in the input.
        // The program should still handle this correctly.
        writeln!(input, "1 {wallet}").unwrap();
    }

    run(
        &input,
        // expected output for price = 1 and wallet = 1 to 100
        // for (1, 1) or (1, 2), we can buy 1 pack
        // for (1, 3) or (1, 4) or (1, 5), we can buy 2 packs
        // etc...
        "\
        1\n1\n\
        2\n2\n2\n\
        3\n3\n3\n3\n\
        4\n4\n4\n4\n4\n\
        5\n5\n5\n5\n5\n5\n\
        6\n6\n6\n6\n6\n6\n6\n\
        7\n7\n7\n7\n7\n7\n7\n7\n\
        8\n8\n8\n8\n8\n8\n8\n8\n8\n\
        9\n9\n9\n9\n9\n9\n9\n9\n9\n9\n\
        10\n10\n10\n10\n10\n10\n10\n10\n10\n10\n10\n\
        11\n11\n11\n11\n11\n11\n11\n11\n11\n11\n11\n11\n\
        12\n12\n12\n12\n12\n12\n12\n12\n12\n12\n12\n12\n12\n\
        13\n13\n13\n13\n13\n13\n13\n13\n13\n13\n",
    );
    // This is rather tedious, so we delegate further testing of `max_packs` to `max_packs.rs`.
}
