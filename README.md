# SPCC Kickstart Solutions

This repository contains a set of solutions to SPCC Kickstart problems.

## Downloading

The following procedures may require familiarity with the comamnd line.
If you need a simpler way to download, please file an issue.

1. [Install Git](https://git-scm.com/downloads) if it hasn't been installed.

2. Install _nightly_ Rust using `rustup` from
   [Rust's official installation page](https://www.rust-lang.org/tools/install).
   
3. Clone this repository:
   ```
   git clone https://github.com/ivan-shrimp/spcc_kickstart.git
   ```

## Usage

`cd` into the `spcc_kickstart` directory, then run:
```
cargo run --release --bin <insert lowercase problem number here>
```

For example, to use the solution for problem A1, run:
```
cargo run --release --bin a1
```

As all programs take input via _standard input_, you won't see any output with
just the above.
To actually get some output, either:
- type `SPCC` followed by `Ctrl+Z`, which gives `Hello SPCC!`; or
- run `echo SPCC | cargo run --release --bin a1`, which also gives
  `Hello SPCC!`.

## Project structure

See [ARCHITECTURE.md](ARCHITECTURE.md).

## License

These solutions for SPCC Kickstart are distributed under the terms of both the
MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

The problems and official solutions for SPCC Kickstart are written by SPCC's
Computer Club, and are distributed to their discretion.
