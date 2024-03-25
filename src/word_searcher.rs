// static ESCAPE_CHAR: &str = "\\";
static ALTERNATION: &str = "|";
// static QUESTION_MARK: &str = "?";
// static PLUS_SIGN: &str = "+";
// static ASTERISK: &str = "*";
static DOT_MARK: char = '.';

pub struct Searcher{}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher{}
    }

    pub fn search(&self, pattern: &String ,text: &String) -> Vec<String> {
        let mut resp: Vec<String> = Vec::new();
        let lines:Vec<&str> = text.split("\n").collect();

        for line in lines {
            if self.pattern_match_line(pattern, line.to_string()) {
                resp.push(line.to_string());
            }
        }

        resp
    }

    fn pattern_match_line(&self, pattern: &String, line: String) -> bool {
        if pattern.contains(ALTERNATION) && self.has_alternation(pattern, &line) {
            return true
        }
        if pattern.contains(DOT_MARK) && self.has_dot_wildcards(pattern, &line) {
            return true
        }
        false
    }

    fn has_alternation(&self, pattern: &String, line: &String) -> bool {
        let values = pattern.split(ALTERNATION);
        for value in values {
            if line.contains(value) {
                return true
            }
        }
        false
    }

    fn has_dot_wildcards(&self, pattern: &String, line: &String) -> bool{
        let mut line_chars = line.chars();
        let mut str_match = String::new();
        while let Some(line_char) = line_chars.next() {
            str_match.push(line_char);
            if str_match.len() == pattern.len() && self.check_if_match(&str_match, pattern){
                return true
            }

            if str_match.len() == pattern.len() {
                str_match.clear();
            }
        }
        false
    }

    fn check_if_match(&self, str_match: &String, pattern: &String) -> bool {
        let mut pattern_chars = pattern.chars().peekable();
        let mut str_match_chars = str_match.chars().peekable();
        while let Some(pattern_char) = pattern_chars.next() {
            let c = str_match_chars.next();
            if pattern_char != DOT_MARK && Some(pattern_char) != c{
                return false
            }
        }

        true
    }

}

#[cfg(test)]
mod tests{
    use super::Searcher;

    #[test]
    fn success_alternation() {
        let word = String::from("hola|mundo");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un mensaje hola\n ejemplo mundo hola");
        assert_eq!(vec!["ejemplo de un mensaje hola", " ejemplo mundo hola"], searcher.search(&word, &line));
    }

    #[test]
    fn empty_line_alternation() {
        let word = String::from("hola|mundo");
        let searcher = Searcher::new();
        let line = String::from("");
        let expected: Vec<String> = vec![];
        assert_eq!(expected, searcher.search(&word, &line));
    }

    #[test]
    fn no_matches_alternation() {
        let word = String::from("hola|mundo");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto");
        let expected: Vec<String> = vec![];
        assert_eq!(expected, searcher.search(&word, &line));
    }

    #[test]
    fn one_dot_match_one() {
        let word = String::from("ej.mplo");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto de cinco letras");
        assert_eq!(vec!["ejemplo de un texto de cinco letras"], searcher.search(&word, &line));
    }

    #[test]
    fn all_characters_are_dots() {
        let word = String::from(".....");
        let searcher = Searcher::new();
        let line = String::from("ejemplo de un texto \nde cinco letras");
        assert_eq!(vec!["ejemplo de un texto", "de cinco letras"], searcher.search(&word, &line));
    }

}
