#[doc(hidden)]
pub fn run(package_name: &str, input: &str, output: &'static str) {
    assert_cmd::Command::cargo_bin(package_name)
        .unwrap()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(output);
}

/// Creates a `run` function that ensures that the program produces `output` when given `input`.
/// This is a macro to make sure that the `env` macro expands within the test.
///
/// Note that this function cannot be used with `miri`.
/// Declare `#![cfg(not(miri))]` at the top of the test file.
#[macro_export]
macro_rules! initialize {
    () => {
        /// Runs the main executable and writes `input` to stdin,
        /// then ensures that it produces `output` in stdout.
        ///
        /// # Panics
        ///
        /// Panics when the program does not produce `output`.
        fn run(input: &str, output: &'static str) {
            $crate::run(env!("CARGO_PKG_NAME"), input, output);
        }
    };
}
