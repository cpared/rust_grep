use crate::regex::RegexChar;
use crate::bracket_expresion::{self, *};

// static ESCAPE_CHAR: &str = "\\";
static ALTERNATION: &str = "|";
// static QUESTION_MARK: &str = "?";
// static PLUS_SIGN: &str = "+";
static ASTERISK: char = '*';
static DOT_MARK: char = '.';
static CLOSED_BRAKET: char = ']';
static DASH: char = '-';
static NEGATED_BRAKET_SIMBOL: char = '^';
static COLON: char = ':';

#[derive(Debug)]
pub struct Searcher{}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher{}
    }

    pub fn search(&self, pattern: &str ,text: &str) -> Vec<String> {
        let mut resp: Vec<String> = Vec::new();
        let lines:Vec<&str> = text.split("\n").collect();

        for line in lines {
            if self.pattern_match_line(pattern, line) {
                resp.push(line.to_string());
            }
        }

        resp
    }

    fn pattern_match_line(&self, pattern: &str, line: &str) -> bool {
        let mut pattern_array :Vec<&str> = Vec::new();
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
                        let mut negate = false;
                        let mut matched = false;
                        if regex_pattern.peek() == Some(&NEGATED_BRAKET_SIMBOL) {
                            negate = true;
                            regex_pattern.next();
                        }
        
                        while let Some(&regex_char) = regex_pattern.next() {
                            if regex_char == CLOSED_BRAKET {
                                break;
                            }
        
                            if regex_char == COLON {
                                if !class_name.is_empty() {
                                    class_name.clear();
                                }
                                while let Some(&class_c) = regex_pattern.next() {
                                    if class_c == COLON && regex_pattern.peek() == Some(&CLOSED_BRAKET) {
                                        regex_pattern.next();
                                        break;
                                    } else {
                                        class_name.push(class_c);
                                    }
                                }
                                if let Some(&lc) = line_iter.peek() {
                                    if is_char_in_class(lc, &class_name) != negate {
                                        matched = true;
                                    }
                                }
                            } else if let Some(&lc) = line_iter.peek() {
                                if (lc == regex_char) != negate {
                                    matched = true;
                                }
                            }
                        }
        
                        if !matched {
                            return false;
                        } else {
                            line_iter.next();
                        }
                    }
                    '*' => {
                        if let Some(previous_char) = regex_pattern.previous() {
                            if previous_char == &DOT_MARK {
                                let remaining_pattern = regex_pattern.remaining_pattern();
                                let original_pos = line_iter.pos();
                                let mut temp_pos = original_pos;

                                while temp_pos <= line.len() {
                                    if self.pattern_match_line(&remaining_pattern, &line[temp_pos..]) {
                                        line_iter.set_pos(temp_pos);
                                        return true;
                                    }
                                    temp_pos += 1;
                                }
                                
                                return false;
                            }
                            
                            let mut matched = false;
                            while line_iter.peek() == Some(previous_char) || regex_pattern.peek() == Some(&ASTERISK) {
                                line_iter.next();
                                matched = true;
                            }
                            if !matched {
                                return false;
                            }
                        }
                    }
                    '+' => {
                        if let Some(previous_char) = regex_pattern.previous() {
                            if previous_char == &CLOSED_BRAKET && !class_name.is_empty() {
                                while let Some(&next_char) = line_iter.peek() {
                                    if !is_char_in_class(next_char, &class_name) {
                                        return false;
                                    }
                                    line_iter.next();
                                }
                                return true;
                            }

                            let mut amount_matched = 1;
                            while let Some(next_char) = line_iter.peek() {
                                if next_char == previous_char {
                                    line_iter.next();
                                    amount_matched += 1;
                                } else {
                                    break;
                                }
                            }
                            if amount_matched <= 1 {
                                return false;
                            }
                        }
                    }
                    '$' => {
                        if !regex_pattern.next().is_none() {
                            return false;
                        }
                        let slice = &pattern[..pattern.len()-1];
                        return line.ends_with(slice)
                    }
                    '{' => {
                        if let Some(&previous) = regex_pattern.previous() {
                            let mut range :Vec<usize> = Vec::new();
                            let mut limitless = false;
                            let mut matched = false;
                            while let Some(rc) = regex_pattern.next() {
                                match rc {
                                    '}' => {
                                        break;
                                    }
                                    ',' => {
                                        if let Some(rc_next) = regex_pattern.next() {
                                            if rc_next.is_digit(10) {
                                                range.push((*rc_next as usize) - ('0' as usize));
                                                regex_pattern.next();
                                            } else {
                                                limitless = true;
                                            }
                                            break;
                                        }
                                    }
                                    _ => {
                                        if rc.is_digit(10) {
                                            range.push((*rc as usize) - ('0' as usize));
                                        } else {
                                            return false;
                                        }
                                    }
                                }
                            }

                            if range.len() == 1 {
                                let mut amount_matched = 1;
                                for _ in 0..range[0] {
                                    if let Some(lc) = line_iter.peek() {
                                        if lc == &previous {
                                            amount_matched += 1;
                                        } else {
                                            break;
                                        }
                                        line_iter.next();
                                    }
                                }
                                matched = amount_matched == range[0];
                            }
                            if range.len() == 2 {
                                let mut amount_matched = 1;
                                for _ in 0..range[1] {
                                    if let Some(lc) = line_iter.peek() {
                                        if lc == &previous {
                                            amount_matched += 1;
                                        } else {
                                            break;
                                        }
                                        line_iter.next();
                                    }
                                }
                                matched = amount_matched >= range[0] && amount_matched <= range[1];
                            }
                            if !matched {
                                return false;
                            }
                        }

                    }
                    '?' => {
                        if let Some(previous_char) = regex_pattern.previous() {
                            if line_iter.peek() == Some(previous_char) {
                                line_iter.next();
                            }
                        }
                    },
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
mod tests{
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
        assert_eq!(vec!["ejemplo de un mensaje hola", " ejemplo mundo hola"], searcher.search(&word, &line));
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
        assert_eq!(vec!["ejemplo de un texto de cinco letras"], searcher.search(&word, &line));
    }

    #[test]
    fn all_characters_are_dots() {
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto \nde cinco letras");
        assert_eq!(vec!["ejemplo de un texto ", "de cinco letras"], searcher.search(".....", &line));
    }

    #[test]
    fn complex_pattern_test() {
        let searcher = Searcher::new();
        assert_eq!(vec!["aXYZb"], searcher.search("a.*b|c?d+", "aXYZb"));
        assert_eq!(vec!["cd"], searcher.search("a.*b|c?d+", "cd"));
        assert_eq!(vec!["aXYZc"], searcher.search("a.*b|c?d+", "aXYZc"));
    }

}
