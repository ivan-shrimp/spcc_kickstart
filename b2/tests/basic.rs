kickstart_macros::fn_run!(b2);

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
