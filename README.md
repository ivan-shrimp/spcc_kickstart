# SPCC Kickstart Solutions

This repository contains a set of solutions to SPCC Kickstart problems.

## Design decisions

### Why so much code?

These solutions prioritize correctness, maintainability and speed
over programmer time, so all code is commented in detail and
with complete test suites. Some may include benchmarks.

In these solutions, we disregard the fact that SPCC Kickstart
is a competitive programming competition,
and consider edge cases carefully, 
making sure that the programs satisfy all requirements.

As of mid March 2022, no solutions in this repository have been submitted,
because it often requires weeks to write one round of solutions.
The organizers are also unlikely to accept Rust or have Rust installed.
Some solutions require a small number of runtime dependencies
outside the standard library,
which are generally disallowed in competitive programming.

### Why Rust?

There aren't advanced Rust skills in these solutions, 
so they can easily be translated into C++.  
However, the tooling (lints/testing/benchmarking/formatting) 
is generally better with Rust, so the code is in Rust.


## Installation

The following procedures may require familiarity with the comamnd line.

1. [Install Git](https://git-scm.com/downloads) if it hasn't been installed.

2. Install Rust using `rustup` from
   [Rust's official installation page](https://www.rust-lang.org/tools/install).
   (There are some benchmarks that require nightly Rust to compile.
   To use nightly Rust, choose the `nightly` channel when installing with `rustup`.)
   
3. Clone this repository:
   ```
   git clone https://github.com/ivan_shrimp/spcc_kickstart.git
   ```


## Running the binaries

Run each executable with:
```
cargo run --release --bin a1
```

Expected output of the above when `SPCC` is provided to stdin:
```
Hello SPCC!
```

For testing, run either `cargo test` or [`cargo nextest`](https://nexte.st/index.html) 
in the root directory.

For benchmarking: run either `cargo bench` or [`cargo criterion`](https://github.com/bheisler/cargo-criterion) in the root directory.


## License

These solutions for SPCC Kickstart are distributed under the terms of 
both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

The problems and official solutions for SPCC Kickstart are owned by
SPCC's Computer Club, and are distributed to their discretion.
