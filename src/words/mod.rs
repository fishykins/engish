use rand::{distr::weighted::WeightedIndex, prelude::Distribution, rngs::ThreadRng};

#[cfg(feature = "nouns")]
mod propper_nouns;
#[cfg(feature = "nouns")]
pub use propper_nouns::*;

/// Word length by frequency.
const WORD_LENGTH_FREQUENCY: [f32; 15] = [
    0.02998, 0.17651, 0.20511, 0.14787, 0.107, 0.08388, 0.07939, 0.05943, 0.04437, 0.03076,
    0.01761, 0.00958, 0.00518, 0.00222, 0.00076,
];

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
    fn build(&self, rng: &mut ThreadRng) -> String {
        let i = rand_word_length(rng);
        self.build_length(i, rng)
    }
    /// Builds a new word of given length, using the provided rng. Only uses upper-case letters for propper nouns etc.
    fn build_length(&self, length: WordLength, rng: &mut ThreadRng) -> String;
}

static mut WORD_LENGTH_WEIGHTS: Option<WeightedIndex<f32>> = None;

/// Returns the weighted Indices of the average word length.
pub fn word_length_weights() -> WeightedIndex<f32> {
    unsafe {
        if WORD_LENGTH_WEIGHTS.is_none() {
            let weights = WeightedIndex::new(&WORD_LENGTH_FREQUENCY).unwrap();
            WORD_LENGTH_WEIGHTS = Some(weights);
        }
        WORD_LENGTH_WEIGHTS.clone().unwrap()
    }
}

/// A quick and dirty random number generator that uses word length frequencies of the english language.
pub fn rand_word_length(rng: &mut ThreadRng) -> WordLength {
    let i = word_length_weights().sample(rng) + 1;
    return WordLength::Chars(i as u8);
}
