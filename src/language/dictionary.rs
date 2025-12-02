use std::any::{Any, TypeId};
use std::collections::HashMap;
use rand::seq::IndexedRandom;
use super::any_word::{AnyWord};
use super::{Adjective, Noun, Verb, Word}; // Assuming Adjective is in the same module

/// A dictionary of words, categorized by their type. Useful for random word sampling.
#[derive(Default)]
pub struct Dictionary {
    words: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl Dictionary {
    /// Creates a new, empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a word to the dictionary.
    /// The word must implement the `Word` trait and be 'static.
    pub fn add_word<T: Word + 'static>(&mut self, word: T) {
        let type_id = TypeId::of::<T>();
        let entry = self.words.entry(type_id).or_default();
        entry.push(Box::new(word));
    }

    /// Adds multiple words to the dictionary.
    /// The words must implement the `Word` trait and be 'static.
    pub fn add_words<T: Word + 'static>(&mut self, words: Vec<T>) {
        let type_id = TypeId::of::<T>();
        let entry = self.words.entry(type_id).or_default();
        entry.extend(words.into_iter().map(|word| Box::new(word) as Box<dyn Any>));
    }

    /// Retrieves all words of a specific type.
    ///
    /// # Example
    /// ```
    /// # use engish::language::{Dictionary, Noun};
    /// let mut dict = Dictionary::new();
    /// dict.add_word(Noun::new_proper("Gandalf"));
    ///
    /// let nouns = dict.get_words::<Noun>();
    /// assert_eq!(nouns.len(), 1);
    /// assert_eq!(nouns[0].as_ref(), "Gandalf");
    /// ```
    pub fn get_words<T: Word + 'static>(&self) -> Vec<&T> {
        self.words
            .get(&TypeId::of::<T>())
            .map(|words| {
                words
                    .iter()
                    .filter_map(|word| word.downcast_ref::<T>())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Chooses a random word of a specific type from the dictionary.
    pub fn choose_word<'a, T: Word + 'static>(
        &'a self,
        rng: &mut impl rand::Rng,
    ) -> Option<&'a T> {
        self.words
            .get(&TypeId::of::<T>())
            .and_then(|words| words.choose(rng))
            .and_then(|word| word.downcast_ref::<T>())
    }

    /// Chooses a random word of a specific type that matches a given predicate.
    pub fn choose_word_filtered<T: Word + 'static, F>(
        &self,
        filter: F,
        rng: &mut impl rand::Rng,
    ) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        let filtered_words = self.get_words_filtered(filter);
        filtered_words.choose(rng).map(|&word| word)
    }

    /// Retrieves all words of a specific type that match a given predicate.
    ///
    /// # Example
    /// ```
    /// # use engish::language::{Dictionary, Noun};
    /// let mut dict = Dictionary::new();
    /// dict.add_word(Noun::new_proper("Gandalf"));
    /// dict.add_word(Noun::new_common("wizard"));
    ///
    /// // Get only the proper nouns
    /// let proper_nouns = dict.get_words_filtered::<Noun, _>(|n| n.is_proper());
    /// assert_eq!(proper_nouns.len(), 1);
    /// assert_eq!(proper_nouns[0].as_ref(), "Gandalf");
    /// ```
    pub fn get_words_filtered<T: Word + 'static, F>(&self, filter: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.words
            .get(&TypeId::of::<T>())
            .map(|words| {
                words
                    .iter()
                    .filter_map(|word| word.downcast_ref::<T>())
                    .filter(|word| filter(word))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl serde::Serialize for Dictionary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializable_words = Vec::new();

        // This part needs to be updated if new Word types are added.
        for noun in self.get_words::<Noun>() {
            serializable_words.push(AnyWord::from(noun.clone()));
        }
        for verb in self.get_words::<Verb>() {
            serializable_words.push(AnyWord::from(verb.clone()));
        }
        for adjective in self.get_words::<Adjective>() {
            serializable_words.push(AnyWord::from(adjective.clone()));
        }

        serializable_words.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Dictionary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serializable_words = Vec::<AnyWord>::deserialize(deserializer)?;
        let mut dict = Dictionary::new();

        for any_word in serializable_words {
            // Match on the serializable enum directly to get concrete types
            // and add them to the dictionary. This avoids invalid casting.
            match any_word {
                AnyWord::Noun(noun) => dict.add_word(noun),
                AnyWord::Verb(verb) => dict.add_word(verb),
                AnyWord::Adjective(adjective) => dict.add_word(adjective),
            }
        }
        Ok(dict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::Verb;
    use crate::language::Noun;

    #[test]
    fn dictionary_test() {
        let mut dict = Dictionary::new();
        dict.add_word(Noun::new_proper("bilbo"));
        dict.add_word(Noun::new_common("ring"));

        let nouns = dict.get_words::<Noun>();
        assert_eq!(nouns.len(), 2);
        assert_eq!(nouns[0].as_ref(), "Bilbo");
        assert_eq!(nouns[1].as_ref(), "ring");

        // We can also check for types that haven't been added.
        let strings = dict.get_words::<String>();
        assert!(strings.is_empty());
    }

    #[test]
    fn dictionary_filter_test() {
        let mut dict = Dictionary::new();
        dict.add_word(Noun::new_proper("bilbo"));
        dict.add_word(Noun::new_common("ring"));
        dict.add_word(Noun::new_collective("fellowship"));

        let common_nouns = dict.get_words_filtered::<Noun, _>(|n| n.is_common());
        assert_eq!(common_nouns.len(), 1);
        assert_eq!(common_nouns[0].as_ref(), "ring");

        let proper_nouns = dict.get_words_filtered::<Noun, _>(|n| n.is_proper());
        assert_eq!(proper_nouns.len(), 1);
        assert_eq!(proper_nouns[0].as_ref(), "Bilbo");

        let collective_nouns = dict.get_words_filtered::<Noun, _>(|n| n.is_collective());
        assert_eq!(collective_nouns.len(), 1);
        assert_eq!(collective_nouns[0].as_ref(), "fellowship");
    }

    #[test]
    fn dictionary_random_choice_test() {
        let mut dict = Dictionary::new();
        dict.add_word(Noun::new_proper("Aragorn"));
        dict.add_word(Noun::new_common("king"));
        dict.add_word(Verb::new_regular("walk"));

        let mut rng = rand::rng();

        // Test random choice
        let random_noun = dict.choose_word::<Noun>(&mut rng);
        assert!(random_noun.is_some());

        // Test filtered random choice
        let random_proper_noun =
            dict.choose_word_filtered::<Noun, _>(|n| n.is_proper(), &mut rng);
        assert!(random_proper_noun.is_some());
        assert_eq!(random_proper_noun.unwrap().as_ref(), "Aragorn");
    }
}
