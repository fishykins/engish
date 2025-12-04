use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A collection of letters that are grouped together, such as vowels, consonants, etc.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LetterGroup {
    /// The letters in this group
    pub letters: BTreeSet<char>,
    /// does a word require one of these?
    pub required: bool,
}

impl LetterGroup {
    /// Vowels of the English language.
    pub fn vowels() -> Self {
        Self {
            letters: BTreeSet::from(['a', 'e', 'i', 'o', 'u']),
            required: true,
        }
    }

    /// Consonants of the English language. 
    pub fn consonants() -> Self {
        Self {
            letters: BTreeSet::from([
                'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't',
                'v', 'w', 'x', 'y', 'z',
            ]),
            required: false,
        }
    }
}
