use serde::{Deserialize, Serialize};


/// A collection of letters that are grouped together, such as vowels, consonants, etc.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LetterGroup {
    /// The letters in this group
    pub letters: Vec<char>,
    /// does a word require one of these? 
    pub required: bool
}