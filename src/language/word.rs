/// Determines a words length, either in raw characters or syllables.
#[derive(Debug, Clone, Default)]
pub enum WordLength {
    /// No length.
    #[default]
    None,
    /// Length in characters.
    Chars(u8),
    /// Length in syllables.
    Syllables(u8),
}

/// A word!
pub trait Word: Clone + Into<String> {
    /// Make the first letter a capital letter. 
    fn capitalize(&self) -> String {
        let s: String = self.clone().into();
        let mut c = s.chars();
        match c.next() {
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }
}
