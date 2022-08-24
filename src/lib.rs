mod ngrams;
pub use ngrams::*;

#[cfg(feature = "words")]
pub mod words;

pub const VOWLES: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
