use serde::{Deserialize, Serialize};

use super::{Noun, Verb, Adjective};

/// A serializable enum that can represent any known word type.
/// This is used for serializing and deserializing a `Dictionary`.
#[derive(Serialize, Deserialize)]
pub(crate) enum AnyWord {
    Noun(Noun),
    Verb(Verb),
    Adjective(Adjective),
}

impl From<Noun> for AnyWord {
    fn from(noun: Noun) -> Self {
        AnyWord::Noun(noun)
    }
}

impl From<Verb> for AnyWord {
    fn from(verb: Verb) -> Self {
        AnyWord::Verb(verb)
    }
}

impl From<Adjective> for AnyWord {
    fn from(adjective: Adjective) -> Self {
        AnyWord::Adjective(adjective)
    }
}


impl From<AnyWord> for Box<dyn super::Word + 'static> {
    fn from(any_word: AnyWord) -> Self {
        match any_word {
            AnyWord::Noun(noun) => Box::new(noun),
            AnyWord::Verb(verb) => Box::new(verb),
            AnyWord::Adjective(adjective) => Box::new(adjective),
        }
    }
}