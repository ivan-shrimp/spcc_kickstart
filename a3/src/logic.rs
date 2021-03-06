/// Returns the number of vowels in the given input string.
pub fn count_vowels(input: &str) -> usize {
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
    input
        .graphemes(true)
        // If a character is a vowel,
        // then it is said to be contained within the array of all vowels.
        .filter(|&letter| ["a", "e", "i", "o", "u", "A", "E", "I", "O", "U"].contains(&letter))
        .count()
}
