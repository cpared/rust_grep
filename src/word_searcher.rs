use crate::constants::*;
use crate::expressions::*;

#[derive(Default)]
pub struct Searcher {}

impl Searcher {
    pub fn search(&self, pattern: &str, text: &str) -> Vec<String> {
        let mut resp: Vec<String> = Vec::new();
        let lines: Vec<&str> = text.split(LINE_BREAK).collect();

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

        let mut matched: Option<bool> = None;
        for pattern_value in pattern_array {
            let mut class_name = String::new();
            let mut line_iter = regex::RegexChar::new(line);
            let mut regex_pattern = regex::RegexChar::new(pattern_value);
            let mut backtrack_pos: usize = 1;
            let mut matcher = false;

            if regex_pattern.contains(DOLAR_SIGN) {
                regex_pattern.set_pos(regex_pattern.size() - 1);
            }

            while let Some(c) = regex_pattern.next_c() {
                if line_iter.peek().is_none() && c != &CLOSED_BRACES {
                    break;
                }

                match c {
                    '.' => {
                        let next_c = regex_pattern.next_c();
                        if next_c == Some(&ASTERISK) || next_c == Some(&CLOSED_QUESTION_MARK) {
                            regex_pattern.set_pos(regex_pattern.pos() - 1);
                            continue;
                        }
                        regex_pattern.set_pos(regex_pattern.pos() - 1);
                        line_iter.next_c();
                    }
                    '\\' => {
                        if line_iter.peek() != regex_pattern.next_c() {
                            regex_pattern.reset();
                        }
                    }
                    '[' => {
                        if let Some(has_a_match) = bracket_expresion::handle_bracket(
                            &mut regex_pattern,
                            &mut line_iter,
                            &mut class_name,
                        ) {
                            if !has_a_match {
                                regex_pattern.reset();
                            }
                            line_iter.next_c();
                        }
                    }
                    '*' => {
                        if let Some(has_a_match) = repetition::handle_asterisk(
                            &mut regex_pattern,
                            &mut line_iter,
                            line,
                            self,
                        ) {
                            return has_a_match;
                        }
                    }
                    '+' => {
                        if let Some(has_a_match) = repetition::handle_plus(
                            &mut regex_pattern,
                            &mut line_iter,
                            &mut class_name,
                        ) {
                            return has_a_match;
                        }
                    }
                    '$' => {
                        matched = anchoring::handle_dolar_sign(&mut regex_pattern, line);
                        break;
                    }
                    '^' => {
                        matched = anchoring::handle_caret_sign(&mut regex_pattern, line);
                        break;
                    }
                    '{' => {
                        if let Some(has_a_match) =
                            repetition::handle_brace(&mut regex_pattern, &mut line_iter)
                        {
                            if !has_a_match {
                                matcher = false;
                                regex_pattern.reset();
                                line_iter.set_pos(backtrack_pos);
                            }
                        }
                    }
                    '?' => repetition::handle_question_mark(&mut regex_pattern, &mut line_iter),
                    _ => {
                        if let Some(lc) = line_iter.next_c() {
                            if matcher && lc != c {
                                let next_c = regex_pattern.next_c();
                                if next_c == Some(&ASTERISK) || next_c == Some(&CLOSED_BRACES) {
                                    regex_pattern.set_pos(regex_pattern.pos() - 1);
                                    line_iter.set_pos(line_iter.pos() - 1);
                                    continue;
                                } else {
                                    regex_pattern.set_pos(regex_pattern.pos() - 1);
                                }
                                matcher = false;
                                regex_pattern.reset();
                                line_iter.set_pos(backtrack_pos);
                                continue;
                            }
                            if lc == c {
                                matcher = true;
                            }
                            if lc != c {
                                let next_c = regex_pattern.next_c();
                                if next_c == Some(&ASTERISK) || next_c == Some(&CLOSED_BRACES) {
                                    continue;
                                } else {
                                    regex_pattern.set_pos(regex_pattern.pos() - 1);
                                }
                                matcher = false;
                                regex_pattern.reset();
                                continue;
                            }
                        }
                        if regex_pattern.pos() == 1 {
                            backtrack_pos = line_iter.pos();
                        }
                    }
                }
            }

            // Handle $ and ^ signs cases
            if let Some(has_matched) = matched {
                if has_matched {
                    return true;
                } else {
                    continue;
                }
            }

            if regex_pattern.next_c().is_none() && matched.is_none() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_alternation() {
        let searcher = Searcher::default();
        assert_eq!(
            vec!["ejemplo de un mensaje hola", " ejemplo mundo hola"],
            searcher.search(
                "hola|mundo",
                "ejemplo de un mensaje hola\n ejemplo mundo hola"
            )
        );
    }

    #[test]
    fn test_empty_line_alternation() {
        let searcher = Searcher::default();
        let expected: Vec<String> = vec![];
        assert_eq!(expected, searcher.search("hola|mundo", ""));
    }

    #[test]
    fn test_no_matches_alternation() {
        let searcher = Searcher::default();
        let expected: Vec<String> = vec![];
        assert_eq!(
            expected,
            searcher.search("hola|mundo", "ejemplo de un texto")
        );
    }

    #[test]
    fn test_one_dot_match_one() {
        let searcher = Searcher::default();
        assert_eq!(
            vec!["ejemplo de un texto de cinco letras"],
            searcher.search("ej.mplo", "ejemplo de un texto de cinco letras")
        );
    }

    #[test]
    fn test_all_characters_are_dots() {
        let searcher = Searcher::default();
        assert_eq!(
            vec!["ejemplo de un texto ", "de cinco letras"],
            searcher.search(".....", "ejemplo de un texto \nde cinco letras")
        );
    }

    #[test]
    fn test_complex_pattern() {
        let searcher = Searcher::default();
        assert_eq!(vec!["aXYZb"], searcher.search("a.*b|c?d+", "aXYZb"));
        assert_eq!(vec!["aXYZc"], searcher.search("a.*b|c?d+", "aXYZc"));
    }
}
