pub struct RegexChar {
    value: Vec<char>,
    size: usize,
    pos: usize,
}

impl RegexChar {
    pub fn new(str_value: &str) -> Self {
        let value: Vec<char> = str_value.chars().collect();
        let size = value.len();
        RegexChar{
            value: value,
            size: size,
            pos: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn peek(&self) -> Option<&char>{
        if self.pos >= self.size {
            return None
        }
        self.value.get(self.pos)
    }

    pub fn previous(&self) -> Option<&char> {
        if self.pos == 0 {
            return None
        }
        self.value.get(self.pos - 2)
    }

    pub fn next(&mut self) -> Option<&char> {
        if self.pos == self.size {
            return None
        }
        let resp = self.value.get(self.pos);
        self.pos += 1;
        resp
    }
}