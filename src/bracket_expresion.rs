
pub fn is_char_in_class(c: char, class: &str) -> bool {
    match class {
        "alnum" => c.is_alphanumeric(),
        "alpha" => c.is_alphabetic(),
        "digit" => c.is_numeric(),
        "lower" => c.is_lowercase(),
        "upper" => c.is_uppercase(),
        "space" => c.is_whitespace(),
        "punct" => "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".contains(c),
        _ => false,
    }
}

mod test {
    use crate::word_searcher::Searcher;

    #[test]
    fn test_bracket_expression() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["abe"], searcher.search("a[bcd]e", "abe"));
        assert_eq!(vec!["ace"], searcher.search("a[bcd]e", "ace"));
        assert_eq!(expected_empty, searcher.search("a[bcd]e", "aee"));
    }

    #[test]
    fn test_negated_bracket_expression() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["aee"], searcher.search("a[^bcd]e", "aee"));
        assert_eq!(expected_empty, searcher.search("a[^bcd]e", "abe"));
        assert_eq!(expected_empty, searcher.search("a[^bcd]e", "ace"));
    }

    #[test]
    fn test_character_class_digit() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["number: 5"], searcher.search("number: [[:digit:]]", "number: 5"));
        assert_eq!(expected_empty, searcher.search("number: [[:digit:]]", "number: x"));
    }

    #[test]
    fn test_character_class_alpha() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["letter: a"], searcher.search("letter: [[:alpha:]]", "letter: a"));
        assert_eq!(expected_empty, searcher.search("letter: [[:alpha:]]", "number: x"));
    }

    #[test]
    fn test_character_class_alnum() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["input: a"], searcher.search("input: [[:alnum:]]", "input: a"));
        assert_eq!(vec!["input: 1"], searcher.search("input: [[:alnum:]]", "input: 1"));
        assert_eq!(expected_empty, searcher.search("input: [[:alnum:]]", "input: !"));
    }

    #[test]
    fn test_character_class_space() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["space:  "], searcher.search("space: [[:space:]]", "space:  "));
        assert_eq!(expected_empty, searcher.search("space: [[:space:]]", "space:_"));
    }

    #[test]
    fn test_complex_bracket_expression() {
        let searcher = Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["abdf"], searcher.search("a[bc][de]f", "abdf"));
        assert_eq!(vec!["acdf"], searcher.search("a[bc][de]f", "acdf"));
        assert_eq!(expected_empty, searcher.search("a[bc][de]f", "bcca"));
    }

}
