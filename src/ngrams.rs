use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File};
use crate::VOWLES;

#[macro_export]
macro_rules! n_gram(
    ($T: ident, $n: literal) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        pub struct $T {
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

pub trait Frequency {
    fn frequency(&self) -> f32;
}

pub trait AlphabetType {
    fn contains_vowel(&self) -> bool;
    fn contains_consonant(&self) -> bool;
    fn is_consonant(&self) -> bool;
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

    pub fn sample(&self, rng: &mut ThreadRng) -> &T {
        &self.alphabet[self.weights.sample(rng)]
    }

    pub fn sample_vowels(&self, rng: &mut ThreadRng) -> &T {
        &self.alphabet[self.vowels[self.vowel_weights.sample(rng)]]
    }

    pub fn sample_consonants(&self, rng: &mut ThreadRng) -> &T {
        &self.alphabet[self.consonants[self.consonant_weights.sample(rng)]]
    }

    pub fn sample_set(&self) -> Vec<&T> {
        return self.alphabet.iter().map(|x| x).collect()
    }

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
