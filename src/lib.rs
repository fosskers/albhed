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
//! Converting Al Bhed into English:
//! ```
//! let al_bhed = "Oui yvnyet uv dra cay?";
//! let english = al_bhed.chars().filter_map(albhed::from_al_bhed).collect::<String>();
//! assert_eq!("You afraid of the sea?", english);
//! ```
//!
//! Converting English into Al Bhed:
//! ```
//! let english = "Beware of Sandstorms!";
//! let al_bhed = english.chars().filter_map(albhed::to_al_bhed).collect::<String>();
//! assert_eq!("Pafyna uv Cyhtcdunsc!", al_bhed);
//! ```

#![doc(html_root_url = "https://docs.rs/albhed/0.1.0")]

/// The ordering that Al Bhed letters map into English. For instance, `A` in Al
/// Bhed maps to the English `E`.
const AL_BHED: [char; 26] = [
    'E', 'P', 'S', 'T', 'I', 'W', 'K', 'N', 'U', 'V', 'G', 'C', 'L', 'R', 'Y', 'B', 'X', 'H', 'M',
    'D', 'O', 'F', 'Z', 'Q', 'A', 'J',
];

/// The ordering that English letters map into Al Bhed. For instance, `Y` in Al
/// Bhed maps to the English `A`.
const ENGLISH: [char; 26] = [
    'Y', 'P', 'L', 'T', 'A', 'V', 'K', 'R', 'E', 'Z', 'G', 'M', 'S', 'H', 'U', 'B', 'X', 'N', 'C',
    'D', 'I', 'J', 'F', 'Q', 'O', 'W',
];

/// Convert a single `char` from Al Bhed into English.
pub fn from_al_bhed(c: char) -> Option<char> {
    work(&AL_BHED, c)
}

/// Convert a single `char` from English into Al Bhed.
pub fn to_al_bhed(c: char) -> Option<char> {
    work(&ENGLISH, c)
}

fn work(table: &[char; 26], c: char) -> Option<char> {
    match c {
        _ if c.is_ascii_whitespace() || c.is_ascii_digit() || c.is_ascii_punctuation() => Some(c),
        _ if c.is_ascii_alphabetic() => {
            let index: usize = (c.to_digit(36).unwrap() - 10) as usize;
            let trans = table[index];
            if c.is_ascii_lowercase() {
                Some(trans.to_ascii_lowercase())
            } else {
                Some(trans)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let pairs = vec![
            ("Fryd ec drec?", "What is this?"),
            ("Fa gemm ed?", "We kill it?"),
            ("E vunpet ed!", "I forbid it!"),
            ("Cunno.", "Sorry."),
        ];

        pairs.into_iter().for_each(|(ab, eng)| {
            let al_bhed: String = ab.chars().filter_map(from_al_bhed).collect();
            assert_eq!(eng, al_bhed);

            let english: String = eng.chars().filter_map(to_al_bhed).collect();
            assert_eq!(ab, english);
        });
    }
}
