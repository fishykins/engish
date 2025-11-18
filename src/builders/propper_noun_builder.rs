use super::WordBuilder;
use crate::language::{Language, Noun, WordLength};
use crate::util::LetterSampler;
use rand::{distr::weighted::WeightedIndex, prelude::Distribution};

/// An opinionated noun builder that emulates English words.
#[derive(Debug, Clone, Default)]
pub struct PropperNounBuilder {
    language: Language,
}

impl PropperNounBuilder {
    /// Creates a new `EngishPropperNounBuilder` with the given language.
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Returns the `Language` component of this builder.
    pub fn language(&self) -> &Language {
        &self.language
    }
}

impl WordBuilder<Noun> for PropperNounBuilder {
    fn build_length(&self, length: WordLength, rng: &mut impl rand::Rng) -> Noun {
        let main_sampler = LetterSampler::new(&self.language.alphabet);

        let letter_count = match length {
            WordLength::Chars(len) => len as usize,
            WordLength::None => {
                // Use a weighted distribution for more natural word lengths.
                let lengths = [3, 4, 5, 6, 7, 8, 9];
                let weights = [1, 5, 9, 10, 8, 5, 1];
                let dist = WeightedIndex::new(&weights).unwrap();
                lengths[dist.sample(rng)]
            }
        };

        let mut word = Vec::<char>::new();

        let first_letter = main_sampler.sample(rng);
        word.push(first_letter);

        // ============== Main logic ============== //

        while word.len() < letter_count {
            let last = word[word.len() - 1];
            let mut digraph_sampler = LetterSampler::from_digraphs(&self.language.alphabet[&last]);

            // Avoid double letters at the start of the word (e.g. 'aa')
            if word.len() == 1 {
                digraph_sampler.remove_char(last);
                let last_type = self.language.letter_group(last).unwrap();
                digraph_sampler.remove_group(last_type);

                // Prevent double vowels at the start of words.
                if self.language.letter_type(last) == Some("vowels")
                {
                    if let Some(vowels) = self.language.get_group("vowels") {
                        digraph_sampler.remove_group(vowels);
                    }
                }

                // Handle specific awkward starting digraphs.
                if last == 'w' || last == 'N' {
                    if let Some(consonants) = self.language.get_group("consonants") {
                        digraph_sampler.remove_group(consonants);
                        digraph_sampler.add_letters_with_freq(vec![
                            self.language.get_letter_pair_unchecked('r'),
                            self.language.get_letter_pair_unchecked('h')
                        ]);
                    }
                }
            }

            if word.len() >= 2 {
                let second_last = word[word.len() - 2];

                // Avoid triple letters (e.g. 'aaa')
                if last == second_last {
                    digraph_sampler.remove_char(last);
                }

                // Avoid having three vowels or three consonants in a row.
                if let (Some(last_type), Some(second_last_type)) = (
                    self.language.letter_type(last),
                    self.language.letter_type(second_last),
                ) {
                    if last_type == second_last_type {
                        if let Some(group) = self.language.get_group(&last_type) {
                            digraph_sampler.remove_group(group);
                        }
                    }
                }
            }



            let next = digraph_sampler.sample(rng);
            word.push(next);
        }
        // ======================================== //
        let noun: String = word.iter().collect();
        return Noun::new_proper(noun);
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propper_noun_test() {
        let mut rng = rand::rng();
        let nb = PropperNounBuilder::default();

        for i in 0..100 {
            let noun = nb.build(&mut rng);
            println!("{}: {}", i, noun);
        }
    }
}
