kickstart_macros::fn_run!(a4);

#[test]
fn samples() {
    run("15", "5\n");
    run("100", "34\n");
}

// Unfortunately, our tests have to assume that our way of
// climbing stairs is correct.
#[test]
fn edge_cases() {
    run("1", "1\n");
    run("2", "1\n");
    run("3", "1\n");
    run("4", "2\n");

    run("999997", "333333\n");
    run("999998", "333333\n");
    run("999999", "333333\n");
    run("1000000", "333334\n");
}
