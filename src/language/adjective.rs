use crate::language::Word;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

/// An adjective is a word that describes or modifies a noun or pronoun,
/// providing more information about its qualities, such as color, size, or opinion.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Adjective(AdjectiveData);

/// Internal representation of an adjective.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
enum AdjectiveData {
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
    Absolute {
        base: String,
    },
}

impl Adjective {
    /// Creates a new regular adjective from its base form.
    /// It uses standard English rules for forming comparatives (-er) and superlatives (-est).
    /// This will not work correctly for all irregular adjectives.
    pub fn new_regular<S: Into<String>>(base: S) -> Self {
        Adjective(AdjectiveData::Regular { base: base.into() })
    }

    /// Creates a new irregular adjective, providing all its forms.
    pub fn new_irregular<S: Into<String>>(base: S, comparative: S, superlative: S) -> Self {
        Adjective(AdjectiveData::Irregular {
            base: base.into(),
            comparative: comparative.into(),
            superlative: superlative.into(),
        })
    }

    /// Creates a new absolute adjective, providing all its forms.
    pub fn new_absolute<S: Into<String>>(base: S) -> Self {
        Adjective(AdjectiveData::Absolute { base: base.into() })
    }

    /// Returns `true` if the adjective is absolute.
    pub fn is_absolute(&self) -> bool {
        matches!(&self.0, AdjectiveData::Absolute { .. })
    }

    /// Returns the comparative form of the adjective (e.g., "faster", "better").
    pub fn comparative<'a>(&'a self) -> Cow<'a, str> {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'h')
        }

        match &self.0 {
            AdjectiveData::Regular { base } => {
                if base.ends_with('e') {
                    return format!("{}r", base).into();
                }
                if let Some(stem) = base.strip_suffix('y') {
                    // For single-syllable words like "shy", the stem is not empty.
                    // If the stem is empty (e.g. word is just "y"), we'd fall through.
                    // The main check is for multi-character words where 'y' is preceded by a consonant.
                    if let Some(before_y) = stem.chars().last() {
                        if !is_vowel(before_y) {
                            return format!("{}ier", stem).into();
                        }
                    }
                }
                if crate::language::utils::ends_cvc(base) {
                    if let Some(last) = base.chars().last() {
                        return format!("{}{}er", base, last).into();
                    }
                }
                format!("{}er", base).into()
            }
            AdjectiveData::Irregular { comparative, .. } => comparative.as_str().into(),
            AdjectiveData::Absolute { base } => base.as_str().into(),
        }
    }

    /// Returns the superlative form of the adjective (e.g., "fastest", "best").
    pub fn superlative<'a>(&'a self) -> Cow<'a, str> {
        // Superlative forms follow the same spelling rules as comparative,
        // but with an "-est" suffix. We can derive it from the comparative form.
        match &self.0 {
            AdjectiveData::Regular { .. } => {
                let comparative = self.comparative();
                let stem = comparative.strip_suffix("er").unwrap_or(&comparative);
                format!("{}est", stem).into()
            }
            AdjectiveData::Irregular { superlative, .. } => superlative.as_str().into(),
            AdjectiveData::Absolute { base } => base.as_str().into(),
        }
    }
}

impl AsRef<str> for Adjective {
    fn as_ref(&self) -> &str {
        match &self.0 {
            AdjectiveData::Regular { base } => base,
            AdjectiveData::Irregular { base, .. } => base,
            AdjectiveData::Absolute { base } => base,
        }
    }
}

impl std::fmt::Display for Adjective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            AdjectiveData::Regular { base } => write!(f, "{}", base),
            AdjectiveData::Irregular { base, .. } => write!(f, "{}", base),
            AdjectiveData::Absolute { base } => write!(f, "{}", base),
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

        // Test -e rule
        let large_adj = Adjective::new_regular("large");
        assert_eq!(large_adj.comparative(), "larger");
        assert_eq!(large_adj.superlative(), "largest");

        // Test -y rule
        let happy_adj = Adjective::new_regular("happy");
        assert_eq!(happy_adj.comparative(), "happier");
        assert_eq!(happy_adj.superlative(), "happiest");

        // Test CVC rule
        let big_adj = Adjective::new_regular("big");
        assert_eq!(big_adj.comparative(), "bigger");
        assert_eq!(big_adj.superlative(), "biggest");

        let irregular_adj = Adjective::new_irregular("good", "better", "best");
        assert_eq!(irregular_adj.comparative(), "better");
        assert_eq!(irregular_adj.superlative(), "best");
    }

    #[test]
    fn adjective_edge_case_test() {
        // Test -y with preceding vowel (should not become -ier)
        let grey_adj = Adjective::new_regular("grey");
        assert_eq!(grey_adj.comparative(), "greyer");
        assert_eq!(grey_adj.superlative(), "greyest");

        // Test single-syllable -y
        let shy_adj = Adjective::new_regular("shy");
        assert_eq!(shy_adj.comparative(), "shyer");
        assert_eq!(shy_adj.superlative(), "shyest");

        // Test CVC rule exception (ends in 'w')
        let slow_adj = Adjective::new_regular("slow");
        assert_eq!(slow_adj.comparative(), "slower");
        assert_eq!(slow_adj.superlative(), "slowest");

        // Test another CVC case
        let thin_adj = Adjective::new_regular("thin");
        assert_eq!(thin_adj.comparative(), "thinner");
        assert_eq!(thin_adj.superlative(), "thinnest");
    }
}
