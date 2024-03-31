use crate::regex::RegexChar;
use crate::constants::*;

pub fn handle_question_mark(regex_pattern: &mut RegexChar, line_iter: &mut RegexChar) {
    if let Some(previous_char) = regex_pattern.previous() {
        if line_iter.peek() == Some(previous_char) {
            line_iter.next();
        }
    }
}

mod test {
    use crate::word_searcher::Searcher;

    #[test]
    fn test_single_character_repetition() {
        let searcher = Searcher::new();
        assert_eq!(vec!["aaa"], searcher.search("a+", "aaa"));
        assert_eq!(vec!["abc"], searcher.search("ab+c", "abc"));
        assert_eq!(vec!["bba"], searcher.search("a+", "bba"));
    }

    #[test]
    fn test_mixed_characters_and_quantifiers() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["aaabbbccc"], searcher.search("a+b+c+", "aaabbbccc"));
        assert_eq!(vec!["aabbc"], searcher.search("a+b+c+", "aabbc"));
        assert_eq!(expected_empty, searcher.search("a+b+c+", "fgh"));
    }
}