use crate::regex::RegexChar;

pub fn handle_dolar_sign(
    regex_pattern: &mut RegexChar, 
    pattern: &str, 
    line: &str
) -> Option<bool> {
    if regex_pattern.next_c().is_some() {
        return Some(false);
    }
    Some(line.ends_with(&pattern[..pattern.len() - 1]))
}

#[cfg(test)]
mod tests {
    // Seteo la posicion final en cada uno de los test ya que esta funcion 
    // se llama cuando ya se consumio el caracter $
    use super::*;

    #[test]
    fn test_handle_dolar_sign_match_end() {
        let mut regex_pattern = RegexChar::new("test$");
        regex_pattern.set_pos(5);
        assert_eq!(handle_dolar_sign(&mut regex_pattern, "test$", "Esto es un test"), Some(true));
        assert_eq!(handle_dolar_sign(&mut regex_pattern, "test$", "test esta al final"), Some(false));
    }

    #[test]
    fn test_handle_dolar_sign_with_additional_chars() {
        let mut regex_pattern = RegexChar::new("abcd$");
        regex_pattern.set_pos(5);
        assert_eq!(handle_dolar_sign(&mut regex_pattern, "abcd$", "123abcd"), Some(true));
    }

    #[test]
    fn test_handle_dolar_sign_early_in_pattern() {
        let mut regex_pattern = RegexChar::new("$abcd");
        assert_eq!(handle_dolar_sign(&mut regex_pattern, "$abcd", "abcd"), Some(false));
    }

    #[test]
    fn test_handle_dolar_sign_pattern_empty() {
        let mut regex_pattern = RegexChar::new("$");
        assert_eq!(handle_dolar_sign(&mut regex_pattern, "$", ""), Some(false));
    }
}
