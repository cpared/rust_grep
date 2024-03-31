mod test {
    use rust_grep;

    #[test]
    fn test_dot_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["abbcd"], searcher.search("ab.cd", "abbcd"));
        assert_eq!(vec!["abccd"], searcher.search("ab.cd", "abccd"));
    }
    
    #[test]
    fn test_dot_asterisk_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["abbbbbbcd"], searcher.search("ab.*cd", "abbbbbbcd"));
    }
    
    #[test]
    fn test_bracket_expression() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["abd"], searcher.search("a[bc]d", "abd"));
        assert_eq!(vec!["abd"], searcher.search("a[^bc]d", "abd"));
    }
    
    #[test]
    fn test_abcd_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["abd"], searcher.search("a[bc]d", "abd"));
        assert_eq!(vec!["acd"], searcher.search("a[bc]d", "acd"));
    }
    
    #[test]
    fn test_ab2_4cd_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["abbcd"], searcher.search("ab{2}cd", "abbcd"));
        assert_eq!(vec!["abbbcd"], searcher.search("ab{2,4}cd", "abbbcd"));
        assert_eq!(vec!["abbbbcd"], searcher.search("ab{2,4}cd", "abbbbcd"));
    }
    
    #[test]
    fn test_abc_de_f_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["abc"], searcher.search("abc|de+f", "abc"));
        assert_eq!(vec!["deef"], searcher.search("abc|de+f", "deef"));
    }
    
    #[test]
    fn test_aeiou_vocal_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["la e es una vocal"], searcher.search("la [aeiou] es una vocal", "la e es una vocal"));
        assert_eq!(vec!["la a es una vocal"], searcher.search("la [aeiou] es una vocal", "la a es una vocal"));
    }
    
    #[test]
    fn test_aeiou_not_vocal_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["la b no es una vocal"], searcher.search("la [^aeiou] no es una vocal", "la b no es una vocal"));
    }
    
    #[test]
    fn test_hola_alpha_plus_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["hola mundo"], searcher.search("hola [[:alpha:]]+", "hola mundo"));
        assert_eq!(expected_empty, searcher.search("hola [[:alpha:]]+", "hola123"));
    }
    
    #[test]
    fn test_digit_es_un_numero_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["3 es un numero"], searcher.search("[[:digit:]] es un numero", "3 es un numero"));
        assert_eq!(expected_empty, searcher.search("[[:digit:]] es un numero", "a es un numero"));
    }
    
    #[test]
    fn test_alnum_no_es_un_simbolo_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["el caracter 3 no es un simbolo"], searcher.search("el caracter [[:alnum:]] no es un simbolo", "el caracter 3 no es un simbolo"));
        assert_eq!(expected_empty, searcher.search("el caracter [[:alnum:]] no es un simbolo", "el caracter % no es un simbolo"));
    }
    
    #[test]
    fn test_hola_space_mundo_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["hola mundo"], searcher.search("hola[[:space:]]mundo", "hola mundo"));
    }
    
    #[test]
    fn test_upper_ascal_upper_case_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        assert_eq!(vec!["AascalZ"], searcher.search("[[:upper:]]ascal[[:upper:]]", "AascalZ"));
    }
    
    #[test]
    fn test_es_el_fin_pattern() {
        let searcher = rust_grep::word_searcher::Searcher::new();
        let expected_empty: Vec<&str> = Vec::new();
        assert_eq!(vec!["es el fin"], searcher.search("es el fin$", "es el fin"));
        assert_eq!(expected_empty, searcher.search("es el fin$", "es el fin en serio"));
    }
    
}
