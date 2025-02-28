#![warn(missing_docs)]

//! Engish is a library that provides a silly interface for sampling letters and words in an English style.
//! Letter sampling is weighted according to the english language, and support for bigraphs is provided.

/// Various utility functions for general use.
pub mod util;
/// Support for word generation.
#[cfg(feature = "words")]
pub mod words;

/// A collection of tools for building a cutsom language model.
pub mod language;

/// The five major vowels in English.
pub const VOWLES: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
