//! This module contains a collection of sample builders for both languages and words. 
mod letter_sampler;
mod noun_builder_v1;

pub use letter_sampler::*;
pub use noun_builder_v1::*;

use crate::language::{Language, WordLength};
use rand::rngs::ThreadRng;

/// A trait to denote a type that can build words.
pub trait WordBuilder<Word> {
    /// Builds a new word.
    fn build(&self, language: &Language, rng: &mut ThreadRng) -> Word {
        let i = WordLength::Syllables(3);
        self.build_length(language, i, rng)
    }
    /// Builds a new word of given length, using the provided rng. Only uses upper-case letters for propper nouns etc.
    fn build_length(
        &self,
        language: &Language,
        length: WordLength,
        rng: &mut ThreadRng,
    ) -> Word;
}
