use crate::regex::RegexChar;

pub fn handle_dolar_sign(regex_pattern: &mut RegexChar, pattern: &str, line: &str) -> Option<bool> {
    if !regex_pattern.next().is_none() {
        return Some(false);
    }
    let slice = &pattern[..pattern.len() - 1];
    return Some(line.ends_with(slice));
}
