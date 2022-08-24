use crate::VOWLES;
use crate::{Digraph, Letter, NGramSampler};
use rand::{rngs::ThreadRng, Rng};

use super::{WordLength, WordBuilder};

/// Constructs propper nouns.
#[derive(Debug, Clone, Default)]
pub struct NounBuilder {
    digraphs: NGramSampler<Digraph>,
    letters: NGramSampler<Letter>,
}

impl NounBuilder {
    /// Builds a new noun builder.
    pub fn new() -> Self {
        Self {
            digraphs: Default::default(),
            letters: Default::default(),
        }
    }
}

impl WordBuilder for NounBuilder {
    fn build_length(&self, length: WordLength, rng: &mut ThreadRng) -> String {
        let len: usize = match length {
            WordLength::Chars(i) => i as usize,
            WordLength::Syllables(i) => i as usize,
            WordLength::None => 7,
        }.max(3);

        let mut word = Vec::<char>::new();
        while word.len() < len {
            if rng.gen() {
                // Use a letter
                let l = word.len();
                if l > 0 {
                    let last = word[l - 1];
                    if VOWLES.contains(&last) {
                        let new: char = self.letters.sample_consonants(rng).into();
                        if new != last {
                            word.push(new);
                        }
                    } else {
                        word.push(self.letters.sample_vowels(rng).into());
                    }
                } else {
                    word.push(self.letters.sample(rng).into());
                }
            } else {
                word.append(&mut self.digraphs.sample(rng).chars.to_vec());
            }
        }

        let first = word[0].to_ascii_uppercase();
        word[0] = first;
        return word.into_iter().collect();
    }

    
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propper_noun_test() {
        let mut rng = rand::thread_rng();
        let nb = NounBuilder::new();

        for i in 0..100 {
            let noun = nb.build(&mut rng);
            println!("{}: {}", i, noun);
        }
    }
}
