//! A translator for the Al Bhed language.
//!
//! Al Bhed is the language spoken by the Al Bhed people in the land of Spira,
//! the main setting of the game Final Fantasy X.
//!
//! This library doesn't attempt to detect English loanwords that are present in
//! Al Bhed at the time of the game, as in airship/fiend/etc.
//!
//! # Usage
//!
//! ```
//! let al_bhed = "Oui yvnyet uv dra cay?";
//! let english = al_bhed.chars().filter_map(albhed::from_al_bhed).collect::<String>();
//! assert_eq!("You afraid of the sea?", english);
//! ```

// TODO Make sure whitespace and punctuation are handled correctly.

/// The ordering that Al Bhed letters map into English. For instance, `A` in Al
/// Bhed maps to the English `E`.
const AL_BHED: [char; 26] = [
    'E', 'P', 'S', 'T', 'I', 'W', 'K', 'N', 'U', 'V', 'G', 'C', 'L', 'R', 'Y', 'B', 'X', 'H', 'M',
    'D', 'O', 'F', 'Z', 'Q', 'A', 'J',
];

/// Convert a single `char` from Al Bhed into English.
pub fn from_al_bhed(c: char) -> Option<char> {
    match c {
        _ if c.is_ascii_whitespace() || c.is_ascii_digit() || c.is_ascii_punctuation() => Some(c),
        _ if c.is_ascii_alphabetic() => {
            let index: usize = (c.to_digit(36).unwrap() - 10) as usize;
            let eng = AL_BHED[index];
            if c.is_ascii_lowercase() {
                Some(eng.to_ascii_lowercase())
            } else {
                Some(eng)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_test() {
        let pairs = vec![
            ("Fryd ec drec?", "What is this?"),
            ("Fa gemm ed?", "We kill it?"),
            ("E vunpet ed!", "I forbid it!"),
            ("Cunno.", "Sorry."),
        ];

        pairs.into_iter().for_each(|(ab, eng)| {
            let trans: String = ab.chars().filter_map(from_al_bhed).collect();
            assert_eq!(eng, trans);
        });
    }
}
