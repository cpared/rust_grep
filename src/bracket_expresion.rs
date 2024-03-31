use crate::constants::*;
use crate::regex::RegexChar;

pub fn handle_bracket(
    regex_pattern: &mut RegexChar,
    line_iter: &mut RegexChar,
    class_name: &mut String,
) -> Option<bool> {
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
        return Some(false);
    } else {
        line_iter.next();
    }
    None
}

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

mod test {}
