use super::{Letter, LetterGroup};
use rand::{distr::weighted::WeightedIndex, prelude::Distribution, rngs::ThreadRng};
use std::collections::HashMap;

/// A neat little struct to quickly sample letters based on frequency.
#[derive(Debug, Clone)]
pub struct LetterSampler {
    /// The alphabet to sample from.
    pub alphabet: Vec<char>,
    /// The weights of each letter.
    pub weights: WeightedIndex<f32>,
}

impl LetterSampler {
    /// Makes a new letter sampler from the given HashMap.
    pub fn new(alphabet: HashMap<char, Letter>) -> Self {
        let base_weights: Vec<f32> = alphabet.iter().map(|(_, l)| l.frequency).collect();
        let weights = WeightedIndex::new(&base_weights).unwrap();
        let alphabet: Vec<char> = alphabet.keys().cloned().collect();
        Self { alphabet, weights }
    }

    /// Makes a new letter sampler from the given letter's potential digraphs.
    pub fn from_digraphs(letter: &Letter) -> Self {
        let digraphs = letter.digraphs.clone();
        let base_weights: Vec<f32> = digraphs.iter().map(|d| d.frequency).collect();
        let weights = WeightedIndex::new(&base_weights).unwrap();
        let alphabet: Vec<char> = digraphs.iter().map(|d| d.letter).collect();
        Self { alphabet, weights }
    }

    /// Takes a random value using a weighted frequency.
    pub fn sample(&self, rng: &mut ThreadRng) -> char {
        self.alphabet[self.weights.sample(rng)]
    }

    /// Filters out any letters in the given group from this sampler.
    pub fn remove_group(&mut self, group: LetterGroup) {
        let mut new_alphabet = Vec::new();
        let mut new_weights = Vec::new();
        for (i, letter) in self.alphabet.iter().enumerate() {
            if !group.letters.contains(letter) {
                new_alphabet.push(*letter);
                new_weights.push(self.weights.weight(i).unwrap());
            }
        }
        self.alphabet = new_alphabet;
        self.weights = WeightedIndex::new(&new_weights).unwrap();
    }


    /// Removes the given letter from this sampler.
    pub fn remove_char(&mut self, letter: char) {
        let mut new_alphabet = Vec::new();
        let mut new_weights = Vec::new();
        for (i, l) in self.alphabet.iter().enumerate() {
            if *l != letter {
                new_alphabet.push(*l);
                new_weights.push(self.weights.weight(i).unwrap());
            }
        }
        self.alphabet = new_alphabet;    
        self.weights = WeightedIndex::new(&new_weights).unwrap();
    }
}
