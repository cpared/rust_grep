use crate::regex::RegexChar;

pub fn build_range(regex_pattern: &mut RegexChar, range: &mut Vec<usize>) -> Option<bool> {
    while let Some(rc) = regex_pattern.next_c() {
        match rc {
            '}' => break,
            ',' => {
                if let Some(rc_next) = regex_pattern.next_c() {
                    if rc_next.is_ascii_digit() {
                        range.push((*rc_next as usize) - ('0' as usize));
                        regex_pattern.next_c();
                    }
                    break;
                }
            }
            _ => {
                if rc.is_ascii_digit() {
                    range.push((*rc as usize) - ('0' as usize));
                } else {
                    return Some(false);
                }
            }
        }
    }
    None
}

pub fn build_brace_response(
    previous_char: Option<char>,
    range: &mut [usize],
    line_iter: &mut RegexChar,
) -> Option<bool> {
    let mut matched = false;
    if let Some(previous) = previous_char {
        match range.len() {
            1 => {
                matched = get_amount_matched(range[0], previous, line_iter) == range[0];
            }
            2 => {
                let amount_matched = get_amount_matched(range[1], previous, line_iter);
                matched = amount_matched >= range[0] && amount_matched <= range[1];
            }
            _ => (),
        }
        if !matched {
            return Some(false);
        }
    }
    None
}

fn get_amount_matched(max: usize, previous: char, line_iter: &mut RegexChar) -> usize {
    let mut amount_matched = 1;
    for _ in 0..max {
        if let Some(lc) = line_iter.peek() {
            if lc == &previous {
                amount_matched += 1;
            } else {
                break;
            }
            line_iter.next_c();
        }
    }
    amount_matched
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_range_single_digit() {
        let mut regex_char = RegexChar::new("3}");
        let mut range = Vec::new();
        assert_eq!(build_range(&mut regex_char, &mut range), None);
        assert_eq!(range, vec![3]);
    }

    #[test]
    fn test_build_range_invalid_char() {
        let mut regex_char = RegexChar::new("a}");
        let mut range = Vec::new();
        assert_eq!(build_range(&mut regex_char, &mut range), Some(false));
    }

    #[test]
    fn test_build_brace_response_single_length() {
        let mut regex_char = RegexChar::new("aaa");
        regex_char.next_c();
        let mut range = vec![3];
        assert_eq!(build_brace_response(Some('a'), &mut range, &mut regex_char), None);
    }
}
