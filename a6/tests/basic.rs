kickstart_macros::fn_run!(a6);

#[test]
fn sample() {
    run("5\n2 4 8 11 12", "8\n");
    run("4\n2 3 4 5", "-1\n");
}

#[test]
fn repeat() {
    run("3\n2 2 4", "4\n");
    run("3\n2 4 4", "-1\n");
    run("6\n2 2 4 4 16 23", "16\n");
}

#[test]
fn scrambled() {
    run("5\n11 8 2 4 12", "8\n");
    run("4\n5 4 3 2", "-1\n");
}
