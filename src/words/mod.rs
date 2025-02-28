use rand::rngs::ThreadRng;

mod noun_builder_v1;
pub use noun_builder_v1::*;

use crate::language::LanguageModel;

/// Word length by frequency.
// const WORD_LENGTH_FREQUENCY: [f32; 15] = [
//     0.02998, 0.17651, 0.20511, 0.14787, 0.107, 0.08388, 0.07939, 0.05943, 0.04437, 0.03076,
//     0.01761, 0.00958, 0.00518, 0.00222, 0.00076,
// ];

/// Determines a words length, either in raw characters or syllables.
#[derive(Debug, Clone, Default)]
pub enum WordLength {
    /// No length.
    #[default]
    None,
    /// Length in characters.
    Chars(u8),
    /// Length in syllables.
    Syllables(u8),
}

/// A trait to denote a type that can build words.
pub trait WordBuilder {
    /// Builds a new word.
    fn build(&self, language: &LanguageModel, rng: &mut ThreadRng) -> String {
        let i = WordLength::Syllables(3);
        self.build_length(language, i, rng)
    }
    /// Builds a new word of given length, using the provided rng. Only uses upper-case letters for propper nouns etc.
    fn build_length(&self, language: &LanguageModel, length: WordLength, rng: &mut ThreadRng) -> String;
}
