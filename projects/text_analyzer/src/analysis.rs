pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn count_characters(text: &str) -> usize {
    text.chars().filter(|c| !c.is_whitespace()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let text = "Hello world!";
        assert_eq!(count_words(text), 2);
    }

    #[test]
    fn test_count_characters() {
        let text = "Hello world!";
        assert_eq!(count_characters(text), 11); // Without count whitespaces
    }
}