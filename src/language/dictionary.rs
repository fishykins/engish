use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use super::{Adjective, Noun, Verb, Word}; // Assuming Adjective is in the same module

/// A private trait to abstract over word types within the dictionary.
/// This allows generic methods to access the correct `Vec` of words.
pub trait DictionaryWordType: Word + 'static {
    fn get_words(dictionary: &Dictionary) -> &Vec<Self>;
    fn get_words_mut(dictionary: &mut Dictionary) -> &mut Vec<Self>;
}

impl DictionaryWordType for Noun {
    fn get_words(dictionary: &Dictionary) -> &Vec<Self> { &dictionary.nouns }
    fn get_words_mut(dictionary: &mut Dictionary) -> &mut Vec<Self> { &mut dictionary.nouns }
}
impl DictionaryWordType for Verb {
    fn get_words(dictionary: &Dictionary) -> &Vec<Self> { &dictionary.verbs }
    fn get_words_mut(dictionary: &mut Dictionary) -> &mut Vec<Self> { &mut dictionary.verbs }
}
impl DictionaryWordType for Adjective {
    fn get_words(dictionary: &Dictionary) -> &Vec<Self> { &dictionary.adjectives }
    fn get_words_mut(dictionary: &mut Dictionary) -> &mut Vec<Self> { &mut dictionary.adjectives }
}

/// A dictionary of words, categorized by their type. Useful for random word sampling.
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Dictionary {
    nouns: Vec<Noun>,
    verbs: Vec<Verb>,
    adjectives: Vec<Adjective>,
}

impl Dictionary {
    /// Creates a new, empty `Dictionary`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a word to the dictionary.
    /// The word must implement the `Word` trait and be 'static.
    pub fn add_word<T: DictionaryWordType>(&mut self, word: T) {
        T::get_words_mut(self).push(word);
    }

    /// Adds multiple words to the dictionary.
    /// The words must implement the `Word` trait and be 'static.
    pub fn add_words<T: DictionaryWordType>(&mut self, words: Vec<T>) {
        T::get_words_mut(self).extend(words);
    }
}


impl Dictionary {
        /// Retrieves all words of a specific type.
    ///
    /// # Example
    /// ```
    /// # use engish::language::{Dictionary, Noun};
    /// let mut dict = Dictionary::new();
    /// dict.add_word(Noun::new_proper("Gandalf"));
    ///
    /// let nouns = dict.get_all::<Noun>();
    /// assert_eq!(nouns.len(), 1);
    /// assert_eq!(nouns[0].as_ref(), "Gandalf");
    /// ```
    pub fn get_all<T: DictionaryWordType>(&self) -> Vec<&T> {
        T::get_words(self).iter().collect()
    }

    /// Chooses a random word of a specific type from the dictionary.
    pub fn choose<'a, T: DictionaryWordType + 'static>(
        &'a self,
        rng: &mut impl rand::Rng,
    ) -> Option<&'a T> {
        T::get_words(self).choose(rng)
    }

    /// Chooses a random word of a specific type that matches a given predicate.
    pub fn choose_filtered<T: DictionaryWordType, F>(
        &self,
        filter: F,
        rng: &mut impl rand::Rng,
    ) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        let filtered_words: Vec<&T> = T::get_words(self)
            .iter()
            .filter(|&word| filter(word))
            .collect();

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
    /// let proper_nouns = dict.get_filtered::<Noun, _>(|n| n.is_proper());
    /// assert_eq!(proper_nouns.len(), 1);
    /// assert_eq!(proper_nouns[0].as_ref(), "Gandalf");
    /// ```
    pub fn get_filtered<T: DictionaryWordType, F>(&self, filter: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
    {
        T::get_words(self)
            .iter()
            .filter(|&word| filter(word))
            .collect()
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

        let nouns = dict.get_all::<Noun>();
        assert_eq!(nouns.len(), 2);
        assert_eq!(nouns[0].as_ref(), "Bilbo");
        assert_eq!(nouns[1].as_ref(), "ring");
    }

    #[test]
    fn dictionary_filter_test() {
        let mut dict = Dictionary::new();
        dict.add_word(Noun::new_proper("bilbo"));
        dict.add_word(Noun::new_common("ring"));
        dict.add_word(Noun::new_collective("fellowship"));

        let common_nouns = dict.get_filtered::<Noun, _>(|n| n.is_common());
        assert_eq!(common_nouns.len(), 1);
        assert_eq!(common_nouns[0].as_ref(), "ring");

        let proper_nouns = dict.get_filtered::<Noun, _>(|n| n.is_proper());
        assert_eq!(proper_nouns.len(), 1);
        assert_eq!(proper_nouns[0].as_ref(), "Bilbo");

        let collective_nouns = dict.get_filtered::<Noun, _>(|n| n.is_collective());
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
        let random_noun = dict.choose::<Noun>(&mut rng);
        assert!(random_noun.is_some());

        // Test filtered random choice
        let random_proper_noun =
            dict.choose_filtered::<Noun, _>(|n| n.is_proper(), &mut rng);
        assert!(random_proper_noun.is_some());
        assert_eq!(random_proper_noun.unwrap().as_ref(), "Aragorn");
    }

    #[test]
    fn dictionary_clone_test() {
        let mut original_dict = Dictionary::new();
        original_dict.add_word(Noun::new_proper("Frodo"));
        original_dict.add_word(Verb::new_regular("run"));
        original_dict.add_word(Adjective::new_regular("brave"));

        let cloned_dict = original_dict.clone();

        // Verify that the cloned dictionary contains the same words
        let original_nouns = original_dict.get_all::<Noun>();
        let cloned_nouns = cloned_dict.get_all::<Noun>();
        assert_eq!(original_nouns.len(), cloned_nouns.len());
        assert_eq!(original_nouns[0].as_ref(), cloned_nouns[0].as_ref());

        let original_verbs = original_dict.get_all::<Verb>();
        let cloned_verbs = cloned_dict.get_all::<Verb>();
        assert_eq!(original_verbs.len(), cloned_verbs.len());
        assert_eq!(original_verbs[0].as_ref(), cloned_verbs[0].as_ref());

        let original_adjectives = original_dict.get_all::<Adjective>();
        let cloned_adjectives = cloned_dict.get_all::<Adjective>();
        assert_eq!(original_adjectives.len(), cloned_adjectives.len());
        assert_eq!(original_adjectives[0].as_ref(), cloned_adjectives[0].as_ref());
    }
}
