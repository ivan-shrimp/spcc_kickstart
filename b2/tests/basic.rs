#![cfg(not(miri))] // miri can't run other executables

test_run_bin::initialize!();

#[test]
fn sample() {
    run("IN", "YES\n");
    run("IN\n", "YES\n");
}

#[test]
fn yes_or_no() {
    run("YES", "NO\n");
    run("NO", "YES\n");
}
