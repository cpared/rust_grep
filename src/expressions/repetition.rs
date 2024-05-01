use crate::constants::*;
use crate::expressions::bracket_expresion::is_char_in_class;
use crate::expressions::regex::RegexChar;
use crate::utils::repetition_utils;
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
    if let Some(mut previous_char) = regex_pattern.previous() {
        if previous_char == &DOT_MARK {
            let remaining_pattern = regex_pattern.remaining_pattern();
            regex_pattern.set_pos(regex_pattern.pos() - 3);

            while let Some(line_value) = line_iter.next_c() {
                if Some(line_value) != regex_pattern.peek() {
                    break;
                }
            }

            let mut temp_pos = line_iter.pos() -1;
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
        if line_iter.peek() != Some(previous_char) {
            if let Some(next_c) = regex_pattern.peek() {
                previous_char = next_c;
            }
        }
        while line_iter.peek() == Some(previous_char) || regex_pattern.peek() == Some(&ASTERISK) {
            line_iter.next_c();
            matched = true;
        }
        if !matched {
            return Some(false);
        } else {
            return Some(true);
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
    if let Some(has_invalid_char) = repetition_utils::build_range(regex_pattern, &mut range) {
        return Some(has_invalid_char);
    }

    repetition_utils::build_brace_response(previous_char, &mut range, line_iter)
}

pub fn handle_plus(
    regex_pattern: &mut RegexChar,
    line_iter: &mut RegexChar,
    class_name: &mut str,
) -> Option<bool> {
    let mut previous_char = None;
    if let Some(&prev) = regex_pattern.previous() {
        previous_char = Some(prev);
    } else if let Some(&next) = regex_pattern.next_c() {
        if regex_pattern.pos() == 1 {
            previous_char = Some(next);
        }
    }

    if previous_char == Some(CLOSED_BRAKET) && !class_name.is_empty() {
        while let Some(&next_char) = line_iter.peek() {
            if !is_char_in_class(next_char, class_name) {
                return Some(false);
            }
            line_iter.next_c();
        }
        return Some(true);
    }

    let mut amount_matched = 1;
    line_iter.set_pos(line_iter.pos() -1);
    while let Some(&next_char) = line_iter.peek() {
        if Some(next_char) == previous_char {
            line_iter.next_c();
            amount_matched += 1;
        } else {
            break;
        }
    }

    if amount_matched <= 1 {
        return Some(false)
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_within_braces_does_not_match() {
        let mut regex_pattern = RegexChar::new("a{3}");
        let mut line_iter = RegexChar::new("aa");
        assert_eq!(
            handle_brace(&mut regex_pattern, &mut line_iter),
            Some(false)
        );
    }

    #[test]
    fn test_range_within_braces_exceeds_max() {
        let mut regex_pattern = RegexChar::new("a{2,4}");
        let mut line_iter = RegexChar::new("aaaaa");
        assert_eq!(
            handle_brace(&mut regex_pattern, &mut line_iter),
            Some(false)
        );
    }

    #[test]
    fn test_range_within_braces_below_min() {
        let mut regex_pattern = RegexChar::new("a{2,4}");
        let mut line_iter = RegexChar::new("a");
        assert_eq!(
            handle_brace(&mut regex_pattern, &mut line_iter),
            Some(false)
        );
    }

    #[test]
    fn test_range_braces_cero_multiple() {
        let mut regex_pattern = RegexChar::new("c{0,}");
        let mut multiple = RegexChar::new("cccccc");
        assert_eq!(
            handle_brace(&mut regex_pattern, &mut multiple),
            Some(true)
        );
    }

    #[test]
    fn test_range_braces_cero_empty() {
        let mut regex_pattern = RegexChar::new("c{0,}");
        let mut empty = RegexChar::new("");
        assert_eq!(
            handle_brace(&mut regex_pattern, &mut empty),
            Some(true)
        );
    }

    #[test]
    fn test_range_braces_cero_one() {
        let mut regex_pattern = RegexChar::new("c{0,}");
        let mut one = RegexChar::new("c");
        assert_eq!(
            handle_brace(&mut regex_pattern, &mut one),
            Some(true)
        );
    }

}
