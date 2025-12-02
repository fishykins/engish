use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use super::Word;

/// Represents a noun, which can be a person, place, or thing.
#[derive(Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum Noun {
    /// A proper noun, which is always capitalized (e.g., "Bilbo", "London").
    Proper {
        /// The word
        word: String,
    },
    /// A common noun, which is a generic person, place, or thing.
    Common {
        /// The singular word
        singular: String,
        /// The plural word, if applicable.
        plural: Option<String>,
    },
    /// A collective noun, which refers to a group (e.g., "flock", "team").
    Collective {
        /// The singular word for the group (e.g., "flock").
        singular: String,
        /// The plural word for multiple groups (e.g., "flocks").
        plural: Option<String>,
    },
}

impl Display for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Noun::Proper { word } => write!(f, "{}", word),
            Noun::Common { singular, .. } => write!(f, "{}", singular),
            Noun::Collective { singular, .. } => write!(f, "{}", singular),
        }
    }
}

impl Debug for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Proper { word } => f.debug_struct("Proper").field("word", word).finish(),
            Self::Common { singular, plural } => f.debug_struct("Common").field("singular", singular).field("plural", plural).finish(),
            Self::Collective { singular, plural } => {
                f.debug_struct("Collective").field("singular", singular).field("plural", plural).finish()
            }
        }
    }
}

impl Noun {
    /// Creates a new proper noun, which will be capitalized.
    pub fn new_proper<S: Into<String>>(word: S) -> Self {
        Noun::Proper {
            word: word.into().capitalize(),
        }
    }

    /// Creates a new common noun.
    pub fn new_common<S: Into<String>>(singular: S, plural: Option<S>) -> Self {
        Noun::Common {
            singular: singular.into().to_lowercase(),
            plural: plural.map(|s| s.into().to_lowercase()),
        }
    }

    /// Creates a new common noun that is uncounatable.
    pub fn new_uncountable<S: Into<String>>(singular: S) -> Self {
        Noun::Common {
            singular: singular.into().to_lowercase(),
            plural: None,
        }
    }

    /// Creates a new collective noun.
    pub fn new_collective<S: Into<String>>(singular: S, plural: Option<S>) -> Self {
        Noun::Collective {
            singular: singular.into().to_lowercase(),
            plural: plural.map(|s| s.into().to_lowercase()),
        }
    }


    /// Returns the plural form of the noun, if applicable.
    /// For proper nouns, it returns the word itself.
    /// For uncountable common nouns, it returns the singular form.
    pub fn plural(&self) -> &str {
        match self {
            Noun::Proper { word } => word,
            Noun::Common { singular, plural } => plural.as_deref().unwrap_or(singular),
            Noun::Collective { singular, plural } => plural.as_deref().unwrap_or(singular),
        }
    }

    /// Returns `true` if the noun is countable.
    pub fn is_countable(&self) -> bool {
        match self {
            // Proper nouns are generally considered uncountable in a grammatical sense.
            Noun::Proper { .. } => false,
            Noun::Common { plural, .. } => plural.is_some(),
            Noun::Collective { plural, .. } => plural.is_some(),
        }
    }

    /// Returns `true` if the noun is a proper noun.
    pub fn is_proper(&self) -> bool {
        matches!(self, Self::Proper { .. })
    }

    /// Returns `true` if the noun is a common noun.
    pub fn is_common(&self) -> bool {
        matches!(self, Self::Common { .. })
    }

    /// Returns `true` if the noun is a collective noun.
    pub fn is_collective(&self) -> bool {
        matches!(self, Self::Collective { .. })
    }
}

impl AsRef<str> for Noun {
    fn as_ref(&self) -> &str {
        match self {
            Noun::Proper { word } => word,
            Noun::Common { singular, .. } => singular,
            Noun::Collective { singular, .. } => singular,
        }
    }
}

impl From<String> for Noun {
    fn from(s: String) -> Self {
        // By default, we can assume a string becomes a proper noun.
        Self::new_proper(s)
    }
}

impl From<&str> for Noun {
    fn from(s: &str) -> Self {
        // By default, we can assume a string becomes a proper noun.
        Self::new_proper(s)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noun_test() {
        let noun = Noun::new_proper("bilbo");
        assert_eq!(noun.as_ref(), "Bilbo");
    }

    #[test]
    fn noun_from_test() {
        let noun_from_str: Noun = "frodo".into();
        assert_eq!(noun_from_str, Noun::new_proper("frodo"));
        let noun_from_string: Noun = String::from("samwise").into();
        assert_eq!(noun_from_string, Noun::new_proper("samwise"));
    }

    #[test]
    fn common_noun_test() {
        // Countable noun
        let countable_noun = Noun::new_common("dog", Some("dogs"));
        assert_eq!(countable_noun.as_ref(), "dog");
        assert_eq!(countable_noun.plural(), "dogs");
        assert!(countable_noun.is_countable());

        // Uncountable noun
        let uncountable_noun = Noun::new_common("water", None::<&str>);
        assert_eq!(uncountable_noun.as_ref(), "water");
        assert_eq!(uncountable_noun.plural(), "water"); // Plural returns singular
        assert!(!uncountable_noun.is_countable());
    }

    #[test]
    fn collective_noun_test() {
        let collective_noun = Noun::new_collective("flock", Some("flocks"));
        assert_eq!(collective_noun.as_ref(), "flock");
        assert_eq!(collective_noun.plural(), "flocks");
        assert!(collective_noun.is_countable());
    }

    #[test]
    fn noun_type_flags_test() {
        assert!(Noun::new_proper("Gandalf").is_proper());
        assert!(!Noun::new_proper("Gandalf").is_common());
        assert!(Noun::new_common("wizard", None).is_common());
        assert!(Noun::new_collective("fellowship", None).is_collective());
    }

    #[test]
    fn noun_casing_test() {
        assert_eq!(Noun::new_proper("gandalf").as_ref(), "Gandalf");
        assert_eq!(Noun::new_common("Wizard", Some("WIZARDS")).plural(), "wizards");
        assert_eq!(Noun::new_collective("Fellowship", None).as_ref(), "fellowship");
        assert_eq!(Noun::new_uncountable("WATER").as_ref(), "water");
    }
}
