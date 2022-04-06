kickstart_macros::fn_run!(b4);

#[test]
fn sample() {
    run("4\n4 4\n1000000000 0\n7 15\n8 7", "2\n0\n7\n5\n");
}

#[test]
fn zero() {
    run("3\n0 0\n0 1\n1 0", "0\n0\n0\n");
}
