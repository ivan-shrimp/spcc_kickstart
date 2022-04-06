use std::io::prelude::*;

pub fn main_impl(input: impl BufRead, mut output: impl Write) {
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
    let count = input_words(input)
        .graphemes(true)
        // If a character is a vowel,
        // then it is said to be contained within the array of all vowels.
        .filter(|&letter| ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"].contains(&letter))
        .count();

    writeln!(output, "{count}").expect("An output error occured");
}

// Input routine.
fn input_words(input: impl BufRead) -> String {
    input
        // Read a line of input.
        .lines()
        .next()
        .expect("No input was provided")
        .expect("An input error occured")
}
