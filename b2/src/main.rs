fn main() {
    // The 7 characters that will not be changed when rotated 180 degrees.
    const LETTERS_ON_SIGN: &[u8] = b"HINOSXZ";

    // Check that every input letter is a letter that can be put on the sign.
    if input_letters().all(|letter| LETTERS_ON_SIGN.contains(&letter)) {
        println!("YES");
    } else {
        println!("NO");
    }
}

// Input routine.
fn input_letters() -> impl Iterator<Item = u8> {
    use std::io::Read;
    std::io::stdin()
        // Because the input is a string of "all capital letters",
        // we can just iterate through the bytes.
        .bytes()
        // Panic if any I/O error occured.
        .map(|byte| byte.expect("An error occured when attempting to read from stdin"))
        // If it is not a capital letter, we might have reached a newline character; it means we're done.
        .take_while(|&byte| byte.is_ascii_uppercase())
}
