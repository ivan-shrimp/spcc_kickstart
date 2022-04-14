Your friendly guide to reading the code in this repository.

This guide assumes familiarity with basic text-based programming; experience in
Python only is probably good enough, but you'll have a hard time if you know
Scratch only.

## Overview

This project is written in a programming language called _Rust_, known to be
very fast (generally as fast as C and C++, and more than 10x faster than
Python) and easy to maintain and test.

Each solution is written as a Rust _package_ and contained within the
_directories_ (or _folders_) named `a1`, `a2`, `a3`, and so on. Each solution
package contains the following:

- `Cargo.toml`: general information about the package
- `src/`: main source code
  - **`logic.rs`: core logic** (not all packages)
  - `lib.rs`: glue code between the core logic and the _I/O_ (input/output)
  - `main.rs`: asks Rust to generate an executable
- **`tests/`: tests that the correct output is produced**
- **`benches/`: checks that we do not exceed the time limit** (not all packages)

If you're looking for algorithmic stuff, read `logic.rs`. To see how we check
our code to ensure it meets specifications, look at the files in `benches/` and
`tests/`.

> That good Rust projects need multiple files is one of the reasons why Rust
> isn't particularly convenient for competitive programming.

## Rust syntax

> If you wish to learn Rust, read
> [the Rust book](https://doc.rust-lang.org/nightly/book/).

```rust
// This is a line comment.

#[this_is_an_attribute_please_ignore_me]
fn we_will_discuss_functions_in_the_middle() {
    // Variables
    let two = 2;

    // Specifying types
    let three: u32 = 3;
    let three_point_five: f32 = 3.5;

    // Blocks
    let five = { 5 };
    let also_five = { 2; 5 };
    let still_five = { let five = 5; five };

    // Arithmetic
    let sum = 2 + 3;

    // Variables that can be modified
    let mut will_be_seven = 3;
    will_be_seven = 5;
    will_be_seven += 2;

    // Strings
    let hello: &str = "hello";

    // Arrays
    let array: [u32; 3] = [2, 3, 5];
    let three = array[1];

    // Booleans
    let is_true: bool = 2 < 3;

    let mut make_me_two_later = 3;

    // If (selecting one block from two)
    let three = if is_true { make_me_one_later = 2; 3 } else { 7 };

    // If (as regular control flow)
    let mut will_be_five = 3;
    if true {
        will_be_five = 5;
    }

    // For
    let mut will_be_two = 5;
    for number in [1, 2] {
        will_be_two -= number;
    }
    for n in 0..5 {
        // first round n = 0, second round n = 1, ..., last round n = 4
    }

    // Functions: the function body is a block
    fn returns_two() -> u32 { 2 }
    fn add(a: u32, b: u32) -> u32 { a + b }

    let two = returns_two();
    let seven = add(2, 5);

    // Optional values
    let a: Option<u32> = Some(2);
    let b: Option<u32> = None;

    let two = match a {
        Some(n) => n,
        None => 0,
    };

    // Methods: this is a method on Option<u32>
    let same_as_above = a.unwrap_or(0);

    // Closures (lambdas)
    let mut will_be_two = 5;
    let closure = || { will_be_two -= 3; };
    closure();
}
```
