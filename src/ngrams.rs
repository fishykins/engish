use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File};
use crate::VOWLES;

/// A macro used to quickly construct an n-gram type.
#[macro_export]
macro_rules! n_gram(
    ($T: ident, $n: literal) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        /// n-gram type.
        pub struct $T {
            /// The characters contained by this type.
            pub chars: [char; $n],
            frequency: f32,
        }

        impl Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                for c in self.chars.iter() {
                    write!(f, "{}", c)?;
                }
                Ok(())
            }
        }

        impl Frequency for $T {
            fn frequency(&self) -> f32 {
                self.frequency
            }
        }

        impl AlphabetType for $T {
            fn contains_vowel(&self) -> bool {
                VOWLES.iter().any(|v| self.chars.contains(v))
            }

            fn contains_consonant(&self) -> bool {
                VOWLES.iter().any(|v| !self.chars.contains(v))
            }

            fn is_consonant(&self) -> bool {
                self.chars.iter().all(|v| !VOWLES.contains(v))
            }

            fn is_vowel(&self) -> bool {
                self.chars.iter().all(|v| VOWLES.contains(v))
            }
        }
    }
);

/// A trait that annotates anything that can have "frequency".
pub trait Frequency {
    /// Returns the frequency value of self.
    fn frequency(&self) -> f32;
}

/// A trait that annotates something that can be considered alphabetical.
pub trait AlphabetType {
    /// Returns true if any of the characters in this type are vowels.
    fn contains_vowel(&self) -> bool;
    /// Returns true if any of the characters in this type are consonants.
    fn contains_consonant(&self) -> bool;
    /// Returns true if *all* characters in this type are vowels.
    fn is_consonant(&self) -> bool;
    /// Returns true if *all* characters in this type are consonants.
    fn is_vowel(&self) -> bool;
}

n_gram!(Letter, 1);
n_gram!(Digraph, 2);
n_gram!(Trigraph, 3);

impl From<&Letter> for char {
    fn from(letter: &Letter) -> char {
        letter.chars[0]
    }
}

/// A sampler for n-grams.
#[derive(Clone, Debug)]
pub struct NGramSampler<T>
where
    T: Display + Frequency + Clone,
{
    alphabet: Vec<T>,
    weights: WeightedIndex<f32>,
    vowels: Vec<usize>,
    consonants: Vec<usize>,
    vowel_weights: WeightedIndex<f32>,
    consonant_weights: WeightedIndex<f32>,
}

impl<T> NGramSampler<T>
where
    T: Display + Frequency + Clone + AlphabetType,
{
    /// Builds a new sampler using the given alphabet.
    pub fn new(alphabet: Vec<T>) -> Self {
        let l = alphabet.len();
        let mut vowels = Vec::new();
        let mut consonants = Vec::new();
        let mut base_weights = Vec::with_capacity(l);
        let mut base_vowel_weights = Vec::new();
        let mut base_consonant_weights = Vec::new();

        for (i, l) in alphabet.iter().enumerate() {
            base_weights.push(l.frequency());
            if l.is_consonant() {
                consonants.push(i);
                base_consonant_weights.push(l.frequency());
            } else if l.is_vowel() {
                vowels.push(i);
                base_vowel_weights.push(l.frequency());
            }
        }
        let weights = WeightedIndex::new(&base_weights).unwrap();
        let vowel_weights = WeightedIndex::new(&base_vowel_weights).unwrap();
        let consonant_weights = WeightedIndex::new(&base_consonant_weights).unwrap();
        Self {
            alphabet,
            weights,
            vowels,
            consonants,
            consonant_weights,
            vowel_weights,
        }
    }

    /// Takes a random value using a weighted frequency.
    pub fn sample(&self, rng: &mut ThreadRng) -> &T {
        &self.alphabet[self.weights.sample(rng)]
    }

    /// Takes a random vowel, using weight frequencies.
    pub fn sample_vowels(&self, rng: &mut ThreadRng) -> &T {
        &self.alphabet[self.vowels[self.vowel_weights.sample(rng)]]
    }

    /// Takes a random consonant, using weight frequencies.
    pub fn sample_consonants(&self, rng: &mut ThreadRng) -> &T {
        &self.alphabet[self.consonants[self.consonant_weights.sample(rng)]]
    }

    /// Returns a refference to the entire sample set of alphabetical data.
    pub fn sample_set(&self) -> Vec<&T> {
        return self.alphabet.iter().map(|x| x).collect()
    }

    /// Returns the length of the sample set.
    pub fn len(&self) -> usize {
        self.alphabet.len()
    }
}

impl Default for NGramSampler<Letter> {
    fn default() -> Self {
        let input_path = format!("{}/src/letters.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let config: Vec<Letter> = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to load config: {}", e);
            }
        };
        NGramSampler::new(config)
    }
}

impl Default for NGramSampler<Digraph> {
    fn default() -> Self {
        let input_path = format!("{}/src/digraphs.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let config: Vec<Digraph> = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                panic!("Failed to load config: {}", e);
            }
        };
        NGramSampler::new(config)
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_test() {
        let sampler = NGramSampler::<Letter>::default();
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
            let s = sampler.sample_vowels(&mut rng);
            assert!(s.is_vowel());
        }
    }

    #[test]
    fn digram_test() {
        let sampler = NGramSampler::<Digraph>::default();
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
            let s = sampler.sample_consonants(&mut rng);
            assert!(!s.is_vowel());
        }
    }
}
