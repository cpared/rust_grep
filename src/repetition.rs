use crate::bracket_expresion::is_char_in_class;
use crate::constants::*;
use crate::regex::RegexChar;
use crate::word_searcher::Searcher;

pub fn handle_question_mark(regex_pattern: &mut RegexChar, line_iter: &mut RegexChar) {
    if let Some(previous_char) = regex_pattern.previous() {
        if line_iter.peek() == Some(previous_char) {
            line_iter.next();
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
            line_iter.next();
            matched = true;
        }
        if !matched {
            return Some(false);
        }
    }
    None
}

pub fn handle_brace(regex_pattern: &mut RegexChar, line_iter: &mut RegexChar) -> Option<bool> {
    if let Some(&previous) = regex_pattern.previous() {
        let mut range: Vec<usize> = Vec::new();
        let mut matched = false;
        while let Some(rc) = regex_pattern.next() {
            match rc {
                '}' => break,
                ',' => {
                    if let Some(rc_next) = regex_pattern.next() {
                        if rc_next.is_digit(10) {
                            range.push((*rc_next as usize) - ('0' as usize));
                            regex_pattern.next();
                        }
                        break;
                    }
                }
                _ => {
                    if rc.is_digit(10) {
                        range.push((*rc as usize) - ('0' as usize));
                    } else {
                        return Some(false);
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
            return Some(false);
        }
    }
    None
}

pub fn handle_plus(
    regex_pattern: &mut RegexChar,
    line_iter: &mut RegexChar,
    class_name: &mut String,
) -> Option<bool> {
    if let Some(previous_char) = regex_pattern.previous() {
        if previous_char == &CLOSED_BRAKET && !class_name.is_empty() {
            while let Some(&next_char) = line_iter.peek() {
                if !is_char_in_class(next_char, &class_name) {
                    return Some(false);
                }
                line_iter.next();
            }
            return Some(true);
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
            return Some(false);
        }
    }
    None
}

mod test {}
