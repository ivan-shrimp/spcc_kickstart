kickstart_macros::fn_run!(a1);

#[test]
fn samples() {
    run("SPCC", "Hello SPCC!\n");
    run("Computer Club", "Hello Computer Club!\n");
}

#[test]
#[allow(non_snake_case)] // national security
fn Chinese() {
    run("聖保羅", "Hello 聖保羅!\n");
}

#[test]
fn newline() {
    // Unix-style newline
    run("SPCC\n", "Hello SPCC!\n");
    // Windows-style newline
    run("SPCC\r\n", "Hello SPCC!\n");
}
