use std::fmt::Display;

use super::Word;

/// A noun is a word that is a thing.
#[derive(Debug, Clone)]
pub struct Noun {
    word: String,
}

impl Display for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl Noun {
    /// Make a new noun
    pub fn new(word: String) -> Self {
        Self { word }
    }
}

impl Word for Noun {}

impl Into<String> for Noun {
    fn into(self) -> String {
        self.word
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noun_test() {
        let noun = Noun::new("bilbo".to_string());
        assert_eq!(noun.capitalize(), "Bilbo");
    }
}
