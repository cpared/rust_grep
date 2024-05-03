use crate::constants::*;
use crate::expressions::regex::RegexChar;

pub fn handle_bracket(
    regex_pattern: &mut RegexChar,
    line_iter: &mut RegexChar,
    class_name: &mut String,
) -> Option<bool> {
    let mut negate = false;
    let mut matched = false;
    if regex_pattern.peek() == Some(&NEGATED_BRAKET_SIMBOL) {
        negate = true;
        regex_pattern.next_c();
    }

    while let Some(&regex_char) = regex_pattern.next_c() {
        if regex_char == CLOSED_BRAKET {
            break;
        }

        if regex_char == COLON {
            if !class_name.is_empty() {
                class_name.clear();
            }
            while let Some(&class_c) = regex_pattern.next_c() {
                if class_c == COLON && regex_pattern.peek() == Some(&CLOSED_BRAKET) {
                    regex_pattern.next_c();
                    break;
                } else {
                    class_name.push(class_c);
                }
            }
            if let Some(&lc) = line_iter.peek() {
                matched = is_char_in_class(lc, class_name);
            }
        } else if let Some(&lc) = line_iter.peek() {
            if lc == regex_char {
                matched = true;
            }
        }
    }

    if negate {
        return Some(!matched);
    }
    Some(matched)
}

pub fn is_char_in_class(c: char, class: &str) -> bool {
    match class {
        "alnum" => c.is_alphanumeric(),
        "alpha" => c.is_alphabetic(),
        "digit" => c.is_numeric(),
        "lower" => c.is_lowercase(),
        "upper" => c.is_uppercase(),
        "space" => c.is_whitespace(),
        "punct" => c.is_ascii_punctuation(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_char_in_class_with_alnum() {
        assert!(is_char_in_class('a', "alnum"));
        assert!(is_char_in_class('1', "alnum"));
        assert!(!is_char_in_class('?', "alnum"));
    }

    #[test]
    fn test_is_char_in_class_with_alpha() {
        assert!(is_char_in_class('a', "alpha"));
        assert!(!is_char_in_class('1', "alpha"));
    }

    #[test]
    fn test_is_char_in_class_with_digit() {
        assert!(is_char_in_class('2', "digit"));
        assert!(!is_char_in_class('a', "digit"));
    }

    #[test]
    fn test_is_char_in_class_with_lower() {
        assert!(is_char_in_class('a', "lower"));
        assert!(!is_char_in_class('A', "lower"));
    }

    #[test]
    fn test_is_char_in_class_with_upper() {
        assert!(is_char_in_class('A', "upper"));
        assert!(!is_char_in_class('a', "upper"));
    }

    #[test]
    fn test_is_char_in_class_with_space() {
        assert!(is_char_in_class(' ', "space"));
        assert!(is_char_in_class('\t', "space"));
        assert!(!is_char_in_class('a', "space"));
    }

    #[test]
    fn test_is_char_in_class_with_unsupported_class() {
        assert!(!is_char_in_class('a', "xyz"));
    }
}
