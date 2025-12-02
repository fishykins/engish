use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display};

use super::Word;

/// Represents a verb, which describes an action or state.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Verb {
    /// A regular verb that follows standard conjugation rules.
    /// Other forms are generated dynamically from the infinitive.
    Regular {
        /// The base form of the verb (infinitive), e.g., "walk".
        infinitive: String,
    },
    /// An irregular verb where all forms must be specified.
    Irregular {
        /// The base form of the verb (infinitive), e.g., "go".
        infinitive: String,
        /// The present tense, third-person singular form, e.g., "goes".
        present_singular: String,
        /// The past tense form, e.g., "went".
        past: String,
        /// The past participle form, e.g., "gone".
        past_participle: String,
        /// The present participle form (gerund), e.g., "going".
        present_participle: String,
    },
}

impl Verb {
    /// Creates a new irregular verb, providing all its forms.
    ///
    /// # Example
    /// ```
    /// # use engish::language::Verb;
    /// let verb = Verb::new_irregular("go", "goes", "went", "gone", "going");
    /// assert_eq!(verb.as_ref(), "go");
    /// assert_eq!(verb.past(), "went");
    /// ```
    pub fn new_irregular<S: Into<String>>(
        infinitive: S,
        present_singular: S,
        past: S,
        past_participle: S,
        present_participle: S,
    ) -> Self {
        Self::Irregular {
            infinitive: infinitive.into(),
            present_singular: present_singular.into(),
            past: past.into(),
            past_participle: past_participle.into(),
            present_participle: present_participle.into(),
        }
    }

    /// Creates a new regular verb from its infinitive form.
    /// It uses standard English rules for forming tenses (e.g., adding -s, -ed, -ing).
    /// This will not work correctly for all irregular verbs.
    ///
    /// # Example
    /// ```
    /// # use engish::language::Verb;
    /// let verb = Verb::new_regular("walk");
    /// assert_eq!(verb.present_singular, "walks");
    /// assert_eq!(verb.past, "walked");
    /// assert_eq!(verb.past_participle(), "walked");
    /// assert_eq!(verb.present_participle(), "walking");
    /// ```
    pub fn new_regular<S: Into<String>>(infinitive: S) -> Self {
        Self::Regular {
            infinitive: infinitive.into(),
        }
    }

    /// Returns the present tense, third-person singular form.
    pub fn present_singular<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Verb::Regular { infinitive } => format!("{}s", infinitive).into(),
            Verb::Irregular {
                present_singular, ..
            } => present_singular.as_str().into(),
        }
    }

    /// Returns the past tense form.
    pub fn past<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Verb::Regular { infinitive } => format!("{}ed", infinitive).into(),
            Verb::Irregular { past, .. } => past.as_str().into(),
        }
    }

    /// Returns the past participle form.
    pub fn past_participle<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Verb::Regular { infinitive } => format!("{}ed", infinitive).into(),
            Verb::Irregular {
                past_participle, ..
            } => past_participle.as_str().into(),
        }
    }

    /// Returns the present participle form (gerund).
    pub fn present_participle<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Verb::Regular { infinitive } => format!("{}ing", infinitive).into(),
            Verb::Irregular {
                present_participle, ..
            } => present_participle.as_str().into(),
        }
    }
}

impl AsRef<str> for Verb {
    /// Returns the infinitive form of the verb.
    fn as_ref(&self) -> &str {
        match self {
            Verb::Regular { infinitive } => infinitive,
            Verb::Irregular { infinitive, .. } => infinitive,
        }
    }
}

impl Display for Verb {
    /// Displays the infinitive form of the verb.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Verb::Regular { infinitive } => write!(f, "{}", infinitive),
            Verb::Irregular { infinitive, .. } => write!(f, "{}", infinitive),
        }
    }
}

impl Debug for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Regular { infinitive } => f
                .debug_struct("Regular")
                .field("infinitive", infinitive)
                .finish(),
            Self::Irregular {
                infinitive,
                present_singular,
                past,
                past_participle,
                present_participle,
            } => f
                .debug_struct("Irregular")
                .field("infinitive", infinitive)
                .field("present_singular", present_singular)
                .field("past", past)
                .field("past_participle", past_participle)
                .field("present_participle", present_participle)
                .finish(),
        }
    }
}

impl Word for Verb {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verb_test() {
        let regular_verb = Verb::new_regular("play");
        assert_eq!(regular_verb.as_ref(), "play");
        assert_eq!(regular_verb.past(), "played");

        let irregular_verb = Verb::new_irregular("eat", "eats", "ate", "eaten", "eating");
        assert_eq!(irregular_verb.as_ref(), "eat");
        assert_eq!(irregular_verb.past_participle(), "eaten");
    }
}
