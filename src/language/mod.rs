//! This module contains pure data structures that can be used to reference generic languages.
mod adjective;
mod any_word;
mod dictionary;
mod language;
mod letter;
mod letter_group;
mod noun;
mod verb;
mod word;

pub use adjective::Adjective;
pub use dictionary::Dictionary;
pub use language::*;
pub use letter::*;
pub use letter_group::*;
pub use noun::Noun;
pub use verb::Verb;
pub use word::*;

pub(crate) mod utils {

    /// Checks if a word ends in a consonant-vowel-consonant pattern.
    pub(crate) fn ends_cvc(s: &str) -> bool {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
        }
        let mut chars = s.chars().rev();
        if let (Some(c1), Some(c2), Some(c3)) = (chars.next(), chars.next(), chars.next()) {
            // Ends in CVC, but not with w, x, or y
            !is_vowel(c1) && !matches!(c1, 'w' | 'x' | 'y') && is_vowel(c2) && !is_vowel(c3)
        } else {
            false
        }
    }
}
