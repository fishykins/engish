/// Determines a words length, either in raw characters or syllables.
#[derive(Debug, Clone, Default)]
pub enum WordLength {
    /// No length.
    #[default]
    None,
    /// Length in characters.
    Chars(u8),
    
    //Syllables(u8),
}

/// A word!
pub trait Word: AsRef<str> + Clone{
    /// Make the first letter a capital letter. 
    fn capitalize(&self) -> String {
        let s = self.as_ref();
        let mut c = s.chars();
        match c.next() {
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }
}

impl Word for &'static str {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }
}

impl Word for String {
    fn capitalize(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            None => String::new(),
        }
    }
}
