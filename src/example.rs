fn char_matching(&self,
    regex_pattern: &mut RegexChar, 
    line_iter: &mut RegexChar,
    line: &str,
    pattern: &str,
    class_name: &mut String,
) -> bool {
    while let Some(c) = regex_pattern.next_c() {
        match c {
            '.' => { line_iter.next_c(); }
            '\\' => if line_iter.peek() != regex_pattern.next_c() {
                    regex_pattern.reset();
                }
            '[' => if let Some(has_a_match) = handle_bracket(regex_pattern, line_iter, class_name) { 
                    return has_a_match 
                }
            '*' => if let Some(has_a_match) = handle_asterisk(regex_pattern, line_iter, line, self) { 
                    return has_a_match 
                }
            '+' => if let Some(has_a_match) = handle_plus(regex_pattern, line_iter, class_name) { 
                    return has_a_match 
            },
            '$' => if let Some(has_a_match) = handle_dolar_sign(regex_pattern, pattern, line) { 
                    return has_a_match 
            },
            '{' => if let Some(has_a_match) = handle_brace(regex_pattern, line_iter) { 
                    return has_a_match 
            },
            '?' => { handle_question_mark(regex_pattern, line_iter) }
            _ => {
                if let Some(lc) = line_iter.peek() {
                    if lc != c {
                        regex_pattern.reset();
                    }
                }
                if line_iter.next_c().is_none() {
                    line_iter.reset();
                    break;
                }
            }
        }
    }
    false
}