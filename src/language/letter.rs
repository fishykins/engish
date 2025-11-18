use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Data for a letter. As the letter will be stored in a hashmap, we don't actually need to store the actual letter here!
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Letter {
    /// How often this letter appears in the given language set.
    pub frequency: f32,
    /// Any digraphs that start with this letter.
    pub digraphs: Vec<DigraphPair>,
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let digraph_string = self.digraphs.iter().map(|d| d.letter).collect::<String>();

        return write!(
            f,
            "frequency: {}, digraphs: {}",
            self.frequency, digraph_string
        );
    }
}

/// Represents the paired part of a digraph.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DigraphPair {
    /// The letter that ends the digraph.
    pub letter: char,
    /// How often this digraph appears in the given language set.
    pub frequency: f32,
}

/// A simple handle for letter rules.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LetterRule {
    /// The condition for this rule to pass
    pub conditions: Vec<LetterRuleCondition>,
    /// The action to take
    pub action: LetterRuleAction,
    /// The probability of this rule being applied
    #[serde(default)]
    pub probability: Option<f32>,
}

/// Conditions for a letter rule.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LetterRuleCondition {
    /// Always applies.
    Allways,
    /// First letter of a word.
    First,
    /// Last letter of a word.
    Last,
    /// Not first letter of a word.
    NotFirst,
    /// Not last letter of a word.
    NotLast,
    /// If this letter makes up a double letter, looking backwards...
    Double,
    /// Not a double letter, looking backwards...
    NotDouble,
    /// If this letter follows a double letter, such as 'ee' or 'ff'...
    FollowsDouble,
    /// If the letter follows the given char...
    FollowsLetter(char),
    /// If the letter follows any letter from the given letter group...
    FollowsLetterGroup(String),
}

/// Actions for a letter rule.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LetterRuleAction {
    /// Pushes the given char after this letter.
    InsertBefore(char),
    /// Pushes the given char before this letter.
    InsertAfter(char),
    /// Replaces the letter with the given char.
    Replace(char),
    /// Replaces the previous letter with the given char.
    ReplacePrevious(char),
    /// Replaces the next letter with the given char.
    ReplaceNext(char),
    /// Doubles this letter up (pushing the double after it).
    Double,
    /// Doubles up the next letter.
    DoubleNext,
    /// Doubles up the previous letter.
    DoublePrevious,
    /// Outright removes this letter.
    Remove,
    /// Outright removes the next letter.
    RemoveNext,
    /// Outright removes the previous letter.
    RemovePrevious,
}
