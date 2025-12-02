use crate::language::Word;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

/// An adjective is a word that describes or modifies a noun or pronoun, 
/// providing more information about its qualities, such as color, size, or opinion.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum Adjective {
    /// A regular adjective that follows standard rules for comparative and superlative forms.
    /// Other forms are generated dynamically from the base.
    Regular {
        /// The base form of the adjective (e.g., "fast").
        base: String,
    },
    /// An irregular adjective where all forms must be specified.
    Irregular {
        /// The base form of the adjective (e.g., "good").
        base: String,
        /// The comparative form (e.g., "better").
        comparative: String,
        /// The superlative form (e.g., "best").
        superlative: String,
    },
}

impl Adjective {
    /// Creates a new regular adjective from its base form.
    /// It uses standard English rules for forming comparatives (-er) and superlatives (-est).
    /// This will not work correctly for all irregular adjectives.
    pub fn new_regular<S: Into<String>>(base: S) -> Self {
        Self::Regular { base: base.into() }
    }

    /// Creates a new irregular adjective, providing all its forms.
    pub fn new_irregular<S: Into<String>>(base: S, comparative: S, superlative: S) -> Self {
        Self::Irregular {
            base: base.into(),
            comparative: comparative.into(),
            superlative: superlative.into(),
        }
    }

    /// Returns the comparative form of the adjective (e.g., "faster", "better").
    pub fn comparative<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Adjective::Regular { base } => format!("{}er", base).into(),
            Adjective::Irregular { comparative, .. } => comparative.as_str().into(),
        }
    }

    /// Returns the superlative form of the adjective (e.g., "fastest", "best").
    pub fn superlative<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Adjective::Regular { base } => format!("{}est", base).into(),
            Adjective::Irregular { superlative, .. } => superlative.as_str().into(),
        }
    }
}

impl AsRef<str> for Adjective {
    fn as_ref(&self) -> &str {
        match self {
            Adjective::Regular { base } => base,
            Adjective::Irregular { base, .. } => base,
        }
    }
}

impl Word for Adjective {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjective_test() {
        let regular_adj = Adjective::new_regular("quick");
        assert_eq!(regular_adj.comparative(), "quicker");
        assert_eq!(regular_adj.superlative(), "quickest");

        let irregular_adj = Adjective::new_irregular("good", "better", "best");
        assert_eq!(irregular_adj.comparative(), "better");
        assert_eq!(irregular_adj.superlative(), "best");
    }
}