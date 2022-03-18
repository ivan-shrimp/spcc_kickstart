#![cfg(not(miri))] // miri can't run other executables

test_run_bin::initialize!();

const YES: &str = "YES\n";
const NO: &str = "NO\n";

#[test]
fn samples() {
    run("36.5", YES);
    run("37.6", NO);
}

#[test]
fn edge_cases() {
    // lower bound: 36.0 <= x
    run("35.9", NO);
    run("35.99", NO);
    run("36", YES);
    run("36.0", YES);
    run("36.1", YES);

    // upper bound: x < 37.5
    run("37.4", YES);
    run("37.49", YES);
    run("37.5", NO);
    run("37.50", NO);
    run("37.51", NO);
}

#[test]
fn extremes() {
    // the thermometer is in trouble
    run("365", NO);
    run("3.65", NO);
    run("0", NO);
    run("-35.9", NO);
    run("-36.0", NO);
    run("-37.4", NO);
    run("-37.5", NO);
}

#[test]
fn scientific_notation() {
    // these are also floating-point numbers
    run("3.59e1", NO);
    run("3.60e1", YES);
    run("3.74e1", YES);
    run("3.75e1", NO);
}
