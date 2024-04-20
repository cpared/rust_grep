/*
    Casos de prueba dados por la catedra
*/

mod test {
    use rust_grep;

    #[test]
    fn test_dot_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(vec!["abbcd"], searcher.search("ab.cd", "abbcd"));
        assert_eq!(vec!["abccd"], searcher.search("ab.cd", "abccd"));
    }

    #[test]
    fn test_dot_asterisk_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(vec!["abbbbbbcd"], searcher.search("ab.*cd", "abbbbbbcd"));
    }

    #[test]
    fn test_bracket_expression() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(vec!["abd"], searcher.search("a[bc]d", "abd"));
        assert_eq!(vec!["acd"], searcher.search("a[bc]d", "acd"));
        assert_eq!(vec!["abd"], searcher.search("a[^bc]d", "abd"));
    }

    #[test]
    fn test_braces_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(vec!["abbcd"], searcher.search("ab{2}cd", "abbcd"));
        assert_eq!(vec!["abbbcd"], searcher.search("ab{2,4}cd", "abbbcd"));
        assert_eq!(vec!["abbbbcd"], searcher.search("ab{2,4}cd", "abbbbcd"));
    }

    #[test]
    fn test_alternation_plus_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(vec!["abc"], searcher.search("abc|de+f", "abc"));
        assert_eq!(vec!["deef"], searcher.search("abc|de+f", "deef"));
    }

    #[test]
    fn test_bracket_vocal_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["la e es una vocal"],
            searcher.search("la [aeiou] es una vocal", "la e es una vocal")
        );
        assert_eq!(
            vec!["la a es una vocal"],
            searcher.search("la [aeiou] es una vocal", "la a es una vocal")
        );
    }

    #[test]
    fn test_negated_bracket_vocal_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["la b no es una vocal"],
            searcher.search("la [^aeiou] no es una vocal", "la b no es una vocal")
        );
    }

    #[test]
    fn test_alpha_plus_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            vec!["hola mundo"],
            searcher.search("hola [[:alpha:]]+", "hola mundo")
        );
        assert_eq!(
            expected_empty,
            searcher.search("hola [[:alpha:]]+", "hola123")
        );
    }

    #[test]
    fn test_digit_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            vec!["3 es un numero"],
            searcher.search("[[:digit:]] es un numero", "3 es un numero")
        );
        assert_eq!(
            expected_empty,
            searcher.search("[[:digit:]] es un numero", "a es un numero")
        );
    }

    #[test]
    fn test_alnum_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            vec!["el caracter 3 no es un simbolo"],
            searcher.search(
                "el caracter [[:alnum:]] no es un simbolo",
                "el caracter 3 no es un simbolo"
            )
        );
        assert_eq!(
            expected_empty,
            searcher.search(
                "el caracter [[:alnum:]] no es un simbolo",
                "el caracter % no es un simbolo"
            )
        );
    }

    #[test]
    fn test_space_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["hola mundo"],
            searcher.search("hola[[:space:]]mundo", "hola mundo")
        );
    }

    #[test]
    fn test_upper_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["AascalZ"],
            searcher.search("[[:upper:]]ascal[[:upper:]]", "AascalZ")
        );
    }

    #[test]
    fn test_dolar_sign_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            vec!["es el fin"],
            searcher.search("es el fin$", "es el fin")
        );
        assert_eq!(
            expected_empty,
            searcher.search("es el fin$", "es el fin en serio")
        );
    }

    // test realizados luego de la correccion
    #[test]
    fn test_common_aabcd() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["aabcd"],
            searcher.search("abcd", "aabcd")
        );
    }

    #[test]
    fn test_negated_bracket_aaeeiioo() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            expected_empty,
            searcher.search("[^aeiou]", "aaeeiioo")
        );
    }

    #[test]
    fn test_bracket_holahhola5() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["holahhola5"],
            searcher.search("hola[[:digit:]]", "holahhola5")
        );
    }

    #[test]
    fn test_bracket_only_space() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["holl a"],
            searcher.search("[[:space:]]", "holl a")
        );
    }

    #[test]
    fn test_bracket_has_upper_in_the_middle() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["hoLa"],
            searcher.search("[[:upper:]]", "hoLa")
        );
    }

    #[test]
    fn test_bracket_punct() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            vec!["hola."],
            searcher.search("[[:punct:]]", "hola.")
        );
        assert_eq!(
            vec!["ho.la"],
            searcher.search("[[:punct:]]", "ho.la")
        );
        assert_eq!(
            vec![".hola"],
            searcher.search("[[:punct:]]", ".hola")
        );
        assert_eq!(
            expected_empty,
            searcher.search("[[:punct:]]", "hola")
        );
    }

    #[test]
    fn test_plus_abc_plus_d() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        assert_eq!(
            vec!["abcd"],
            searcher.search("abc+d", "abcd")
        );
        assert_eq!(
            vec!["abccd"],
            searcher.search("abc+d", "abccd")
        );
        assert_eq!(
            vec!["aabcd"],
            searcher.search("abc+d", "aabcd")
        );
    }

    #[test]
    fn test_dolar_with_negated_bracket() {
        let searcher = rust_grep::word_searcher::Searcher::default();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(
            vec!["start"],
            searcher.search("^start|end$", "start")
        );
        assert_eq!(
            vec!["end"],
            searcher.search("^start|end$", "end")
        );
        assert_eq!(
            expected_empty,
            searcher.search("^start|end$", "middle")
        );
    }
}
