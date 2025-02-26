use ron::de::from_reader;
use std::collections::HashMap;
use std::fs::File;

use super::{Letter, LetterGroup};

/// A language model.
pub struct Language {
    /// The alphabet of the language.
    pub alphabet: HashMap<char, Letter>,
    /// Collections of letters that are grouped together, such as vowels, consonants, etc.
    pub letter_groups: HashMap<String, LetterGroup>,
}

impl Language {
    /// Checks if the given letter is in the given group.
    pub fn letter_in_group(&self, letter: char, group: &str) -> bool {
        self.letter_groups
            .get(group)
            .map(|g| g.letters.contains(&letter))
            .unwrap_or(false)
    }

    
    /// Returns the letter group this letter belongs to.
    pub fn letter_type(&self, letter: char) -> Option<String> {
        self.letter_groups
            .iter()
            .find_map(|(group, g)| g.letters.contains(&letter).then_some(group.clone()))
    }

    /// Gets the group name of the given letter. 
    /// TODO: handle conditions where it might be in multiple groups. This could be the case with theoretical language models.
    pub fn get_group(&self, group: &str) -> Option<LetterGroup> {
        self.letter_groups.get(group).cloned()
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

impl Default for Language {
    fn default() -> Self {
        let input_path = format!("{}/assets/english_letters.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let alphabet: HashMap<char, Letter> = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to load config: {}", e);
            }
        };
        let input_path = format!("{}/assets/english_letter_groups.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let letter_groups: HashMap<String, LetterGroup> = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to load config: {}", e);
            }
        };

        Self {
            alphabet,
            letter_groups,
        }
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
