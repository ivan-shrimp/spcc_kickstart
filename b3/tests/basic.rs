#![cfg(not(miri))]

test_run_bin::initialize!();

#[test]
fn samples() {
    run("7 1 4", "6\n");
    run("8 1 9", "8\n");
}

#[test]
fn same_place() {
    run("10 10 10", "0\n");
}

#[test]
fn far_away() {
    run("1 10 100", "99\n");
}
