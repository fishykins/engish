//! This module contains pure data structures that can be used to reference generic languages.
mod adjective;
mod any_word;
mod dictionary;
mod language;
mod letter;
mod letter_group;
mod noun;
mod verb;
mod word;

pub use adjective::Adjective;
pub use dictionary::Dictionary;
pub use language::*;
pub use letter::*;
pub use letter_group::*;
pub use noun::Noun;
pub use verb::Verb;
pub use word::*;
