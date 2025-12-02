use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Debug, Display};

use super::Word;

/// Represents a verb, which describes an action or state.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Verb(VerbData);

/// Internal representation of a verb.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename = "Verb")] // Ensure serialization format is stable
enum VerbData {
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
        let kind = VerbData::Irregular {
            infinitive: infinitive.into(),
            present_singular: present_singular.into(),
            past: past.into(),
            past_participle: past_participle.into(),
            present_participle: present_participle.into(),
        };
        Self(kind)
    }

    /// Creates a new regular verb from its infinitive form.
    /// It uses standard English rules for forming tenses (e.g., adding -s, -ed, -ing).
    /// This will not work correctly for all irregular verbs.
    ///
    /// # Example
    /// ```
    /// # use engish::language::Verb;
    /// let verb = Verb::new_regular("walk");
    /// assert_eq!(verb.present_singular(), "walks");
    /// assert_eq!(verb.past(), "walked");
    /// assert_eq!(verb.past_participle(), "walked");
    /// assert_eq!(verb.present_participle(), "walking");
    /// ```
    pub fn new_regular<S: Into<String>>(infinitive: S) -> Self {
        let kind = VerbData::Regular {
            infinitive: infinitive.into(),
        };
        Self(kind)
    }

    /// Returns the present tense, third-person singular form.
    pub fn present_singular<'a>(&'a self) -> Cow<'a, str> {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
        }

        match &self.0 {
            VerbData::Regular { infinitive } => {
                if infinitive.ends_with('y') {
                    let mut chars = infinitive.chars();
                    chars.next_back(); // pop 'y'
                    if let Some(before_y) = chars.next_back() {
                        if !is_vowel(before_y) {
                            return format!("{}ies", &infinitive[..infinitive.len() - 1]).into();
                        }
                    }
                } else if infinitive.ends_with('s')
                    || infinitive.ends_with("sh")
                    || infinitive.ends_with("ch")
                    || infinitive.ends_with('x')
                    || infinitive.ends_with('z')
                {
                    return format!("{}es", infinitive).into();
                }
                format!("{}s", infinitive).into()
            }
            VerbData::Irregular {
                present_singular, ..
            } => present_singular.as_str().into(),
        }
    }

    /// Returns the past tense form.
    pub fn past<'a>(&'a self) -> Cow<'a, str> {
        fn is_vowel(c: char) -> bool {
            matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
        }

        match &self.0 {
            VerbData::Regular { infinitive } => {
                if infinitive.ends_with('e') {
                    return format!("{}d", infinitive).into();
                }
                if infinitive.ends_with('y') {
                    let mut chars = infinitive.chars();
                    chars.next_back(); // pop 'y'
                    if let Some(before_y) = chars.next_back() {
                        if !is_vowel(before_y) {
                            return format!("{}ied", &infinitive[..infinitive.len() - 1]).into();
                        }
                    }
                }
                if crate::language::utils::ends_cvc(infinitive) {
                    if let Some(last) = infinitive.chars().last() {
                        return format!("{}{}ed", infinitive, last).into();
                    }
                }
                format!("{}ed", infinitive).into()
            }
            VerbData::Irregular { past, .. } => past.as_str().into(),
        }
    }

    /// Returns the past participle form.
    pub fn past_participle<'a>(&'a self) -> Cow<'a, str> {
        // For regular verbs, past and past participle are the same.
        match &self.0 {
            VerbData::Regular { .. } => self.past(),
            VerbData::Irregular {
                past_participle, ..
            } => past_participle.as_str().into(),
        }
    }

    /// Returns the present participle form (gerund).
    pub fn present_participle<'a>(&'a self) -> Cow<'a, str> {
        match &self.0 {
            VerbData::Regular { infinitive } => {
                if infinitive == "be" {
                    return "being".into();
                }
                if infinitive.ends_with("ie") {
                    return format!("{}ying", &infinitive[..infinitive.len() - 2]).into();
                }
                if infinitive.ends_with('e') {
                    return format!("{}ing", &infinitive[..infinitive.len() - 1]).into();
                }
                if crate::language::utils::ends_cvc(infinitive) {
                    if let Some(last) = infinitive.chars().last() {
                        return format!("{}{}ing", infinitive, last).into();
                    }
                }
                format!("{}ing", infinitive).into()
            }
            VerbData::Irregular {
                present_participle, ..
            } => present_participle.as_str().into(),
        }
    }
}

impl AsRef<str> for Verb {
    /// Returns the infinitive form of the verb.
    fn as_ref(&self) -> &str {
        match &self.0 {
            VerbData::Regular { infinitive } => infinitive,
            VerbData::Irregular { infinitive, .. } => infinitive,
        }
    }
}

impl Display for Verb {
    /// Displays the infinitive form of the verb.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            VerbData::Regular { infinitive } => write!(f, "{}", infinitive),
            VerbData::Irregular { infinitive, .. } => write!(f, "{}", infinitive),
        }
    }
}

impl Debug for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            VerbData::Regular { infinitive } => f
                .debug_struct("Regular")
                .field("infinitive", infinitive)
                .finish(),
            VerbData::Irregular {
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
        assert_eq!(regular_verb.present_singular(), "plays");
        assert_eq!(regular_verb.past(), "played");

        let irregular_verb = Verb::new_irregular("eat", "eats", "ate", "eaten", "eating");
        assert_eq!(irregular_verb.as_ref(), "eat");
        assert_eq!(irregular_verb.past_participle(), "eaten");

        // Test -y rule
        let study = Verb::new_regular("study");
        assert_eq!(study.present_singular(), "studies");
        assert_eq!(study.past(), "studied");

        // Test -es rule
        assert_eq!(Verb::new_regular("pass").present_singular(), "passes");
        assert_eq!(Verb::new_regular("watch").present_singular(), "watches");
        assert_eq!(Verb::new_regular("fix").present_singular(), "fixes");

        // Test CVC doubling rule
        let stop = Verb::new_regular("stop");
        assert_eq!(stop.past(), "stopped");
        assert_eq!(stop.present_participle(), "stopping");

        // Test -e rule
        assert_eq!(Verb::new_regular("bake").past(), "baked");
        assert_eq!(Verb::new_regular("bake").present_participle(), "baking");
    }

    #[test]
    fn verb_edge_case_test() {
        // Test -ie rule for present participle
        let die = Verb::new_regular("die");
        assert_eq!(die.present_participle(), "dying");
        let lie = Verb::new_regular("lie");
        assert_eq!(lie.present_participle(), "lying");

        // Test -o ending for present singular
        let echo = Verb::new_regular("echo");
        assert_eq!(echo.present_singular(), "echoes");

        // Test CVC rule exceptions (should not double)
        let open = Verb::new_regular("open");
        assert_eq!(open.past(), "opened");
        assert_eq!(open.present_participle(), "opening");

        // Test -z ending
        let buzz = Verb::new_regular("buzz");
        assert_eq!(buzz.present_singular(), "buzzes");
        assert_eq!(buzz.past(), "buzzed"); // CVC rule doesn't apply to 'z'
        assert_eq!(buzz.present_participle(), "buzzing");
    }
}
