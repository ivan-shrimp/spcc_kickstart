#![cfg(not(miri))] // miri can't run other executables

test_run_bin::initialize!();

#[test]
fn sample() {
    run("2\n20 5\n31 6", "YES 4\nNO 1\n");
}

// When the price of the prize is very low.
#[test]
fn cheap() {
    run("3\n1 1\n5 5\n3 6", "YES 1\nYES 1\nNO 3\n");
}

// When the price of the prize is very high.
#[test]
fn expensive() {
    run(
        "3\n1000000 1\n1000000 3\n1000000 1000000",
        "YES 1000000\nNO 1\nYES 1\n",
    );
}
