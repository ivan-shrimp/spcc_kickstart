/// Returns true if every input letter is a letter that can be put on the sign.
pub fn is_sign(letters: impl IntoIterator<Item = u8>) -> bool {
    // The 7 characters that will not be changed when rotated 180 degrees.
    const LETTERS_ON_SIGN: &[u8] = b"HINOSXZ";

    letters
        .into_iter()
        .all(|letter| LETTERS_ON_SIGN.contains(&letter))
}
