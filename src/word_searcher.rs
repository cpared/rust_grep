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

            if regex_pattern.contains('$') {
                regex_pattern.set_pos(regex_pattern.len()-1);
            }

            while let Some(c) = regex_pattern.next_c() {
                match c {
                    '.' => {
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
                            return has_a_match;
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
                        break
                    }
                    '^' => {
                        matched = anchoring::handle_caret_sign(&mut regex_pattern, line);
                        break
                    }
                    '{' => {
                        if let Some(has_a_match) =
                            repetition::handle_brace(&mut regex_pattern, &mut line_iter)
                        {
                            return has_a_match;
                        }
                    }
                    '?' => repetition::handle_question_mark(&mut regex_pattern, &mut line_iter),
                    _ => {
                        if let Some(lc) = line_iter.next_c() {
                            if lc != c {
                                regex_pattern.set_pos(regex_pattern.pos() - 1);
                                continue;
                            }
                        }
                        if line_iter.peek().is_none() && !regex_pattern.contains(DOLAR_SIGN){
                            line_iter.reset();
                            break;
                        }
                    }
                }
            }

            if regex_pattern.next_c().is_none() && matched.is_none() {
                matched = Some(true);
                break;
            }

            if let Some(has_matched) = matched {
                if has_matched {
                    return true
                }
            }
        }
        if let Some(has_matched) = matched {
            return has_matched;
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
