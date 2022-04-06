/// Generates a function that tests the program by calling `main_impl`.
/// ```ignore
/// fn run(input: &str, output: &'static str);
/// ```
#[macro_export]
macro_rules! fn_run {
    ($package:ident) => {
        /// Runs the `main_impl` function and pass `input` as a parameter,
        /// then ensures that it produces `output` in the writer.
        #[track_caller]
        fn run(input: &str, output: &'static str) {
            // Create an output buffer.
            let mut output_writer = Vec::new();

            $package::main_impl(input.as_bytes(), &mut output_writer);

            assert_eq!(&*output_writer, output.as_bytes());
        }
    };
}

/// Generates a `main` function that calls `main_impl` with standard input and output.
#[macro_export]
macro_rules! fn_main {
    ($package:ident) => {
        /// Entry point.
        fn main() {
            $package::main_impl(std::io::stdin().lock(), std::io::stdout().lock());
        }
    };
}
