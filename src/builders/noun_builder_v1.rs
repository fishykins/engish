use super::{WordBuilder, LetterSampler};
use crate::language::{Language, Noun, WordLength};
use rand::prelude::*;

/// Builds nouns.
#[derive(Debug, Clone, Default)]
pub struct NounBuilderV1 {}

impl WordBuilder<Noun> for NounBuilderV1 {
    fn build_length(
        &self,
        language: &Language,
        _length: WordLength,
        rng: &mut ThreadRng,
    ) -> Noun {
        let main_sampler = LetterSampler::new(language.alphabet.clone());
        let mut word = Vec::<char>::new();
        // First, we want to sample the starting letter!
        let first_letter = main_sampler.sample(rng);
        word.push(first_letter);

        // five iterations
        for _ in 0..5 {
            let last = word[word.len() - 1];
            let mut digraph_sampler = LetterSampler::from_digraphs(&language.alphabet[&last]);

            if word.len() >= 2 {
                // Assess the last two entries.
                let last_type = language.letter_type(last).unwrap();
                let second_last = word[word.len() - 2];
                let second_last_type = language.letter_type(second_last).unwrap();

                if last_type == second_last_type {
                    // last two are the same type of letter, lets force it to pick a different type. 
                    digraph_sampler.remove_group(language.get_group(&last_type).unwrap());
                }

                if last == second_last {
                    // last two are the same letter, lets force it to pick a different letter.
                    digraph_sampler.remove_char(last);
                }
            }

            let next = digraph_sampler.sample(rng);
            word.push(next);
        }

        return Noun::new(word.into_iter().collect());
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::language;

    use super::*;

    #[test]
    fn propper_noun_test() {
        let mut rng = rand::rng();
        let language = language::Language::default();
        let nb = NounBuilderV1::default();

        for i in 0..100 {
            let noun = nb.build(&language, &mut rng);
            println!("{}: {}", i, noun);
        }
    }
}
