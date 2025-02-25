/// Prefixes a noun with either "a" or "an".
pub fn add_article<T: AsRef<str> + Into<String>>(noun: T) -> String {
    let noun_str = noun.as_ref().to_lowercase();

    let result = match noun_str.chars().next() {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => {
            format!("an {}", noun_str)
        }
        _ => {
            format!("a {}", noun_str)
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_article_with_consonant() {
        let noun = "apple";
        let expected = "an apple";
        let result = add_article(noun);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_article_with_vowel() {
        let noun = "elephant";
        let expected = "an elephant";
        let result = add_article(noun);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_article_with_empty_string() {
        let noun = "";
        let expected = "a ";
        let result = add_article(noun);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_article_with_single_character() {
        let noun = "o";
        let expected = "an o";
        let result = add_article(noun);
        assert_eq!(result, expected);
    }
}
