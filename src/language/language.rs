use lazy_static::lazy_static;
use ron::de::from_reader;
use std::collections::BTreeMap;
use std::fs::File;

use super::{Letter, LetterGroup};

/// A language model containing all the meta data regarding alphabets, letter groups and other low-level language defining traits.
#[derive(Debug, Clone)]
pub struct Language {
    /// The alphabet of the language.
    pub alphabet: BTreeMap<char, Letter>,
    /// Collections of letters that are grouped together, such as vowels, consonants, etc.
    pub letter_groups: BTreeMap<String, LetterGroup>,
}

impl Language {
    /// Checks if the given letter is in the given group.
    pub fn letter_in_group<S: AsRef<str>>(&self, letter: char, group: S) -> bool {
        self.letter_groups
            .get(group.as_ref())
            .map(|g| g.letters.contains(&letter))
            .unwrap_or(false)
    }

    /// Converts a `char` into a paired data block.
    pub fn get_letter_pair(&self, letter: char) -> Option<(char, &Letter)> {
        if let Some(letter_data) = self.alphabet.get(&letter) {
            return Some((letter, letter_data));
        }
        None
    }

    /// Converts a `char` into a paired data block. Panics if not found!
    /// Useful for working with samplers.
    pub fn get_letter_pair_unchecked(&self, letter: char) -> (char, &Letter) {
        if let Some(letter_data) = self.alphabet.get(&letter) {
            return (letter, letter_data);
        }
        panic!("Letter not found in alphabet: {}", letter)
    }

    /// Returns the letter group type this letter belongs to.
    pub fn letter_type(&self, letter: char) -> Option<&str> {
        self.letter_groups
            .iter()
            .find_map(|(group, g)| g.letters.contains(&letter).then_some(group.as_str()))
    }

    /// Returns the `LetterGroup` for the given `char`, if found.
    pub fn letter_group(&self, letter: char) -> Option<&LetterGroup> {
        self.letter_groups
            .iter()
            .find_map(|(_, g)| g.letters.contains(&letter).then_some(g))
    }

    /// Gets the group name of the given letter.
    /// TODO: handle conditions where it might be in multiple groups. This could be the case with theoretical language models.
    pub fn get_group<S: AsRef<str>>(&self, group: S) -> Option<&LetterGroup> {
        self.letter_groups.get(group.as_ref())
    }

    /// A quick helper function to check if the given letter is a vowel, as defined by the language.
    /// This could return false if the language is configured strangely!
    pub fn is_vowel(&self, letter: char) -> bool {
        self.letter_in_group(letter, "vowels")
    }

    /// A quick helper function to check if the given letter is a consonant, as defined by the language.
    /// This could return false if the language is configured strangely!
    pub fn is_consonant(&self, letter: char) -> bool {
        self.letter_in_group(letter, "consonants")
    }
}

lazy_static! {
    static ref DEFAULT_LANGUAGE: Language = {
        let letters_path = format!("{}/assets/english_letters.ron", env!("CARGO_MANIFEST_DIR"));
        let f_letters = File::open(&letters_path).expect("Failed opening english_letters.ron");
        let alphabet: BTreeMap<char, Letter> =
            from_reader(f_letters).expect("Failed to parse english_letters.ron");

        let groups_path = format!(
            "{}/assets/english_letter_groups.ron",
            env!("CARGO_MANIFEST_DIR")
        );
        let f_groups = File::open(&groups_path).expect("Failed opening english_letter_groups.ron");
        let letter_groups: BTreeMap<String, LetterGroup> =
            from_reader(f_groups).expect("Failed to parse english_letter_groups.ron");

        Language {
            alphabet,
            letter_groups,
        }
    };
}

impl Default for Language {
    fn default() -> Self {
        DEFAULT_LANGUAGE.clone()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_test() {
        let engish = Language::default();
        for (letter, data) in engish.alphabet {
            println!("{}: {}", letter, data);
        }
    }
}
