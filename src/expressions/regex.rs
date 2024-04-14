pub struct RegexChar {
    value: Vec<char>,
    size: usize,
    pos: usize,
}

impl RegexChar {
    pub fn new(str_value: &str) -> Self {
        let value: Vec<char> = str_value.chars().collect();
        let size = value.len();
        RegexChar {
            value,
            size,
            pos: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn peek(&self) -> Option<&char> {
        if self.pos >= self.size {
            return None;
        }
        self.value.get(self.pos)
    }

    pub fn previous(&self) -> Option<&char> {
        if self.pos == 0 {
            return None;
        }
        self.value.get(self.pos - 2)
    }

    pub fn remaining_pattern(&self) -> String {
        self.value[self.pos..].iter().collect()
    }

    pub fn set_pos(&mut self, pos: usize) {
        if pos <= self.size {
            self.pos = pos;
        } else {
            self.pos = self.size;
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn next_c(&mut self) -> Option<&char> {
        if self.pos == self.size {
            return None;
        }
        let resp = self.value.get(self.pos);
        self.pos += 1;
        resp
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let regex_char = RegexChar::new("abc");
        assert_eq!(regex_char.value, vec!['a', 'b', 'c']);
        assert_eq!(regex_char.size, 3);
        assert_eq!(regex_char.pos, 0);
    }

    #[test]
    fn test_next() {
        let mut regex_char = RegexChar::new("abc");
        assert_eq!(regex_char.next_c(), Some(&'a'));
        assert_eq!(regex_char.next_c(), Some(&'b'));
        assert_eq!(regex_char.next_c(), Some(&'c'));
        assert_eq!(regex_char.next_c(), None);
    }

    #[test]
    fn test_reset() {
        let mut regex_char = RegexChar::new("test");
        regex_char.next_c();
        regex_char.reset();
        assert_eq!(regex_char.pos, 0);
    }

    #[test]
    fn test_peek() {
        let regex_char = RegexChar::new("test");
        assert_eq!(regex_char.peek(), Some(&'t'));
    }

    #[test]
    fn test_peek_none() {
        let mut regex_char = RegexChar::new("a");
        regex_char.next_c();
        assert_eq!(regex_char.peek(), None);
    }

    #[test]
    fn test_previous() {
        let mut regex_char = RegexChar::new("abcd");
        regex_char.next_c();
        regex_char.next_c();
        assert_eq!(regex_char.previous(), Some(&'a'));
    }

    #[test]
    fn test_previous_none() {
        let regex_char = RegexChar::new("a");
        assert_eq!(regex_char.previous(), None);
    }

    #[test]
    fn test_remaining_pattern() {
        let mut regex_char = RegexChar::new("abcd");
        regex_char.next_c();
        assert_eq!(regex_char.remaining_pattern(), "bcd");
    }

    #[test]
    fn test_set_pos() {
        let mut regex_char = RegexChar::new("abc");
        regex_char.set_pos(2);
        assert_eq!(regex_char.pos, 2);
    }

    #[test]
    fn test_set_pos_out_of_bounds() {
        let mut regex_char = RegexChar::new("abc");
        regex_char.set_pos(5);
        assert_eq!(regex_char.pos, 3);
    }

    #[test]
    fn test_pos() {
        let regex_char = RegexChar::new("abc");
        assert_eq!(regex_char.pos(), 0);
    }
}
