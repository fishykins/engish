use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display};

use crate::language::Word;

/// Represents the pluralization of a noun.
#[derive(Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Debug)]
pub enum Pluralization {
    /// No form of pluralization.
    None,
    /// An irregular plural form.
    Irregular(String),
    /// A regular plural form.
    Regular,
}

impl Pluralization {
    pub(crate) fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub(crate) fn as_cow<'a>(&'a self, singular: &'a str) -> Cow<'a, str> {
        match self {
            Self::None => Cow::Borrowed(singular),
            Self::Irregular(plural) => Cow::Borrowed(plural),
            Self::Regular => Cow::Owned(format!("{}s", singular)),
        }
    }
}

/// Represents a noun, which can be a person, place, or thing.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Noun(NounData);

/// Internal representation of a Noun.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename = "Noun")]
enum NounData {
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
        plural: Pluralization,
    },
    /// A collective noun, which refers to a group (e.g., "flock", "team").
    Collective {
        /// The singular word for the group (e.g., "flock").
        singular: String,
        /// The plural word for multiple groups (e.g., "flocks").
        plural: Pluralization,
    },
}

impl Display for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            NounData::Proper { word } => write!(f, "{}", word),
            NounData::Common { singular, .. } => write!(f, "{}", singular),
            NounData::Collective { singular, .. } => write!(f, "{}", singular),
        }
    }
}

impl Debug for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            NounData::Proper { word } => f.debug_struct("Proper").field("word", word).finish(),
            NounData::Common { singular, plural } => f
                .debug_struct("Common")
                .field("singular", singular)
                .field("plural", plural)
                .finish(),
            NounData::Collective { singular, plural } => f
                .debug_struct("Collective")
                .field("singular", singular)
                .field("plural", plural)
                .finish(),
        }
    }
}

impl Noun {
    /// Creates a new proper noun, which will be capitalized.
    pub fn new_proper<S: Into<String>>(word: S) -> Self {
        Self(NounData::Proper {
            word: word.into().capitalize(),
        })
    }

    /// Creates a new common noun that follows regular pluralization rules.
    pub fn new_common<S: Into<String>>(singular: S) -> Self {
        Self(NounData::Common {
            singular: singular.into().to_lowercase(),
            plural: Pluralization::Regular,
        })
    }

    /// Creates a new common noun that has an irregular plural form.
    pub fn new_common_irregular<S: Into<String>>(singular: S, plural: S) -> Self {
        Self(NounData::Common {
            singular: singular.into().to_lowercase(),
            plural: Pluralization::Irregular(plural.into().to_lowercase()),
        })
    }

    /// Creates a new common noun that is uncounatable.
    pub fn new_uncountable<S: Into<String>>(singular: S) -> Self {
        Self(NounData::Common {
            singular: singular.into().to_lowercase(),
            plural: Pluralization::None,
        })
    }

    /// Creates a new collective noun that follows regular pluralization rules.
    pub fn new_collective<S: Into<String>>(singular: S) -> Self {
        Self(NounData::Collective {
            singular: singular.into().to_lowercase(),
            plural: Pluralization::Regular,
        })
    }

    /// Creates a new collective noun that follows regular pluralization rules.
    pub fn new_collective_uncountable<S: Into<String>>(singular: S) -> Self {
        Self(NounData::Collective {
            singular: singular.into().to_lowercase(),
            plural: Pluralization::None,
        })
    }

    /// Creates a new collective noun that has an irregular plural form.
    pub fn new_collective_irregular<S: Into<String>>(singular: S, plural: S) -> Self {
        Self(NounData::Collective {
            singular: singular.into().to_lowercase(),
            plural: Pluralization::Irregular(plural.into().to_lowercase()),
        })
    }

    /// Returns the plural form of the noun, if applicable.
    /// For proper nouns, it returns the word itself.
    /// For uncountable common nouns, it returns the singular form.
    pub fn plural<'a>(&'a self) -> Cow<'a, str> {
        match &self.0 {
            NounData::Proper { word } => Cow::Borrowed(&word),
            NounData::Common { singular, plural } => plural.as_cow(&singular),
            NounData::Collective { singular, plural } => plural.as_cow(&singular),
        }
    }

    /// Returns `true` if the noun is countable.
    pub fn is_countable(&self) -> bool {
        match &self.0 {
            // Proper nouns are generally considered uncountable in a grammatical sense.
            NounData::Proper { .. } => false,
            NounData::Common { plural, .. } => plural.is_some(),
            NounData::Collective { plural, .. } => plural.is_some(),
        }
    }

    /// Returns `true` if the noun is a proper noun.
    pub fn is_proper(&self) -> bool {
        matches!(&self.0, NounData::Proper { .. })
    }

    /// Returns `true` if the noun is a common noun.
    pub fn is_common(&self) -> bool {
        matches!(&self.0, NounData::Common { .. })
    }

    /// Returns `true` if the noun is a collective noun.
    pub fn is_collective(&self) -> bool {
        matches!(&self.0, NounData::Collective { .. })
    }
}

impl AsRef<str> for Noun {
    fn as_ref(&self) -> &str {
        match &self.0 {
            NounData::Proper { word } => word,
            NounData::Common { singular, .. } => singular,
            NounData::Collective { singular, .. } => singular,
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
        let countable_noun = Noun::new_common_irregular("dog", "dogs");
        assert_eq!(countable_noun.as_ref(), "dog");
        assert_eq!(countable_noun.plural(), "dogs");
        assert!(countable_noun.is_countable());

        // Uncountable noun
        let uncountable_noun = Noun::new_uncountable("water");
        assert_eq!(uncountable_noun.as_ref(), "water");
        assert_eq!(uncountable_noun.plural(), "water"); // Plural returns singular
        assert!(!uncountable_noun.is_countable());
    }

    #[test]
    fn collective_noun_test() {
        let collective_noun = Noun::new_collective_irregular("flock", "flocks");
        assert_eq!(collective_noun.as_ref(), "flock");
        assert_eq!(collective_noun.plural(), "flocks");
        assert!(collective_noun.is_countable());
    }

    #[test]
    fn noun_type_flags_test() {
        assert!(Noun::new_proper("Gandalf").is_proper());
        assert!(!Noun::new_proper("Gandalf").is_common());
        assert!(Noun::new_uncountable("wizard").is_common());
        assert!(Noun::new_collective("fellowship").is_collective());
    }

    #[test]
    fn noun_casing_test() {
        assert_eq!(Noun::new_proper("gandalf").as_ref(), "Gandalf");
        assert_eq!(
            Noun::new_common_irregular("Wizard", "WIZARDS").plural(),
            "wizards"
        );
        assert_eq!(Noun::new_collective("Fellowship").as_ref(), "fellowship");
        assert_eq!(Noun::new_uncountable("WATER").as_ref(), "water");
    }
}
