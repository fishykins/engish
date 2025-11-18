//! This module contains a collection of sample builders for both languages and words.
mod propper_noun_builder;

pub use propper_noun_builder::*;
use crate::language::WordLength;

/// A trait to denote a type that can build words.
pub trait WordBuilder<Word> {
    /// Builds a new word.
    fn build(&self, rng: &mut impl rand::Rng) -> Word {
        let i = WordLength::None;
        self.build_length(i, rng)
    }
    /// Builds a new word of given length, using the provided rng. Only uses upper-case letters for propper nouns etc.
    fn build_length(&self, length: WordLength, rng: &mut impl rand::Rng) -> Word;
}
