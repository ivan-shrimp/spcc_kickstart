fn main() {
    use unicode_segmentation::UnicodeSegmentation;

    // Iterate over the characters in the string,
    // use `filter` to consider only the vowels,
    // and count them.
    //
    // Some strings look like one character
    // but are represented by multiple Unicode "code points".
    // For example, "é" is actually a combination of the regular "e"
    // and \u{0301} ("combining acute accent").
    // When we count vowels, we should exclude such "character"s
    // as they are not one of 'a', 'e', 'i', 'o', 'u' (or their uppercase counterparts).
    // This is why we use `graphemes`, that iterates over "grapheme clusters".
    // For how that works, see
    // http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries.
    let count = input_words()
        .graphemes(true)
        // If a character is a vowel,
        // then it is said to be contained within the array of all vowels.
        .filter(|&letter| ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"].contains(&letter))
        .count();

    println!("{count}");
}

// Input routine.
fn input_words() -> String {
    use std::io::{self, BufRead};

    // Obtain a handle to stdin.
    io::stdin()
        .lock()
        // Read a line of input.
        .lines()
        .next()
        // Panic if we can't get anything from stdin.
        .expect("No input was provided through stdin")
        // Panic if any I/O error occured.
        .expect("An error occured when attempting to read from stdin")
}
