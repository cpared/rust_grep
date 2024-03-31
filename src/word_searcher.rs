use crate::anchoring::*;
use crate::bracket_expresion::*;
use crate::constants::*;
use crate::regex::RegexChar;
use crate::repetition::*;

#[derive(Debug)]
pub struct Searcher {}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {}
    }

    pub fn search(&self, pattern: &str, text: &str) -> Vec<String> {
        let mut resp: Vec<String> = Vec::new();
        let lines: Vec<&str> = text.split("\n").collect();

        for line in lines {
            if self.pattern_match_line(pattern, line) {
                resp.push(line.to_string());
            }
        }

        resp
    }

    pub fn pattern_match_line(&self, pattern: &str, line: &str) -> bool {
        let mut pattern_array: Vec<&str> = Vec::new();
        if pattern.contains(ALTERNATION) {
            let values = pattern.split(ALTERNATION);
            for value in values {
                pattern_array.push(value);
            }
        } else {
            pattern_array.push(pattern);
        }

        let mut matched = false;
        for pattern_value in pattern_array {
            let mut class_name = String::new();
            let mut line_iter = RegexChar::new(line);
            let mut regex_pattern = RegexChar::new(pattern_value);
            while let Some(c) = regex_pattern.next() {
                match c {
                    '.' => {
                        line_iter.next();
                    }
                    '[' => {
                        match handle_bracket(&mut regex_pattern, &mut line_iter, &mut class_name) {
                            Some(has_a_match) => return has_a_match,
                            None => (),
                        }
                    }
                    '*' => {
                        // This function has a recursive call 🟨
                        match handle_asterisk(&mut regex_pattern, &mut line_iter, line, self) {
                            Some(has_a_match) => return has_a_match,
                            None => (),
                        }
                    }
                    '+' => match handle_plus(&mut regex_pattern, &mut line_iter, &mut class_name) {
                        Some(has_a_match) => return has_a_match,
                        None => (),
                    },
                    '$' => match handle_dolar_sign(&mut regex_pattern, pattern, line) {
                        Some(has_a_match) => return has_a_match,
                        None => (),
                    },
                    '{' => match handle_brace(&mut regex_pattern, &mut line_iter) {
                        Some(has_a_match) => return has_a_match,
                        None => (),
                    },
                    '?' => {
                        handle_question_mark(&mut regex_pattern, &mut line_iter);
                    }
                    _ => {
                        if let Some(lc) = line_iter.peek() {
                            if lc != c {
                                regex_pattern.reset();
                            }
                        }
                        if line_iter.next().is_none() {
                            line_iter.reset();
                            break;
                        }
                    }
                }
            }
            if regex_pattern.next().is_none() {
                matched = true;
                break;
            }
        }
        matched
    }
}

#[cfg(test)]
mod tests {
    use super::Searcher;

    #[test]
    fn test_combination_with_other_characters() {
        let searcher = Searcher::new();
        assert!(searcher.pattern_match_line("a+b?c+", "aabc"));
        assert!(searcher.pattern_match_line("a+b?c+", "aac"));
        assert!(!searcher.pattern_match_line("a+b?c+", "abcabc"));
    }

    #[test]
    fn success_alternation() {
        let word = String::from("hola|mundo");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un mensaje hola\n ejemplo mundo hola");
        assert_eq!(
            vec!["ejemplo de un mensaje hola", " ejemplo mundo hola"],
            searcher.search(&word, &line)
        );
    }

    #[test]
    fn empty_line_alternation() {
        let word = String::from("hola|mundo");
        let searcher = Searcher::new();
        let line = String::from("");
        let expected: Vec<String> = vec![];
        assert_eq!(expected, searcher.search(&word, &line));
    }

    #[test]
    fn no_matches_alternation() {
        let word = String::from("hola|mundo");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto");
        let expected: Vec<String> = vec![];
        assert_eq!(expected, searcher.search(&word, &line));
    }

    #[test]
    fn one_dot_match_one() {
        let word = String::from("ej.mplo");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto de cinco letras");
        assert_eq!(
            vec!["ejemplo de un texto de cinco letras"],
            searcher.search(&word, &line)
        );
    }

    #[test]
    fn all_characters_are_dots() {
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto \nde cinco letras");
        assert_eq!(
            vec!["ejemplo de un texto ", "de cinco letras"],
            searcher.search(".....", &line)
        );
    }

    #[test]
    fn complex_pattern_test() {
        let searcher = Searcher::new();
        assert_eq!(vec!["aXYZb"], searcher.search("a.*b|c?d+", "aXYZb"));
        assert_eq!(vec!["cd"], searcher.search("a.*b|c?d+", "cd"));
        assert_eq!(vec!["aXYZc"], searcher.search("a.*b|c?d+", "aXYZc"));
    }
}
