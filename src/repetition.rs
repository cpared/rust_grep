use crate::bracket_expresion::is_char_in_class;
use crate::constants::*;
use crate::regex::RegexChar;
use crate::repetition_utils::*;
use crate::word_searcher::Searcher;

pub fn handle_question_mark(regex_pattern: &mut RegexChar, line_iter: &mut RegexChar) {
    if let Some(previous_char) = regex_pattern.previous() {
        if line_iter.peek() == Some(previous_char) {
            line_iter.next_c();
        }
    }
}

pub fn handle_asterisk(
    regex_pattern: &mut RegexChar,
    line_iter: &mut RegexChar,
    line: &str,
    searcher: &Searcher,
) -> Option<bool> {
    if let Some(previous_char) = regex_pattern.previous() {
        if previous_char == &DOT_MARK {
            let remaining_pattern = regex_pattern.remaining_pattern();
            let mut temp_pos = line_iter.pos();

            while temp_pos <= line.len() {
                if searcher.pattern_match_line(&remaining_pattern, &line[temp_pos..]) {
                    line_iter.set_pos(temp_pos);
                    return Some(true);
                }
                temp_pos += 1;
            }
            return Some(false);
        }

        let mut matched = false;
        while line_iter.peek() == Some(previous_char) || regex_pattern.peek() == Some(&ASTERISK) {
            line_iter.next_c();
            matched = true;
        }
        if !matched {
            return Some(false);
        }
    }
    None
}

pub fn handle_brace(regex_pattern: &mut RegexChar, line_iter: &mut RegexChar) -> Option<bool> {
    let mut previous_char = None;
    if let Some(&prev) = regex_pattern.previous() {
        previous_char = Some(prev);
    } else if let Some(&next) = regex_pattern.next_c() {
        if regex_pattern.pos() == 1 {
            previous_char = Some(next);
        }
    }

    let mut range: Vec<usize> = Vec::new();
    if let Some(has_invalid_char) = build_range(regex_pattern, &mut range) {
        return Some(has_invalid_char);
    }

    build_brace_response(previous_char, &mut range, line_iter)
}

pub fn handle_plus(
    regex_pattern: &mut RegexChar,
    line_iter: &mut RegexChar,
    class_name: &mut str,
) -> Option<bool> {
    if let Some(previous_char) = regex_pattern.previous() {
        if previous_char == &CLOSED_BRAKET && !class_name.is_empty() {
            while let Some(&next_char) = line_iter.peek() {
                if !is_char_in_class(next_char, class_name) {
                    return Some(false);
                }
                line_iter.next_c();
            }
            return Some(true);
        }

        let mut amount_matched = 1;
        while let Some(next_char) = line_iter.peek() {
            if next_char == previous_char {
                line_iter.next_c();
                amount_matched += 1;
            } else {
                break;
            }
        }
        if amount_matched <= 1 {
            return Some(false);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_within_braces_does_not_match() {
        let mut regex_pattern = RegexChar::new("a{3}");
        let mut line_iter = RegexChar::new("aa");
        assert_eq!(handle_brace(&mut regex_pattern, &mut line_iter), Some(false));
    }

    #[test]
    fn range_within_braces_exceeds_max() {
        let mut regex_pattern = RegexChar::new("a{2,4}");
        let mut line_iter = RegexChar::new("aaaaa");
        assert_eq!(handle_brace(&mut regex_pattern, &mut line_iter), Some(false));
    }

    #[test]
    fn range_within_braces_below_min() {
        let mut regex_pattern = RegexChar::new("a{2,4}");
        let mut line_iter = RegexChar::new("a");
        assert_eq!(handle_brace(&mut regex_pattern, &mut line_iter), Some(false));
    }
}
