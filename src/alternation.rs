
struct Alternation {}

#[derive(default)]
impl Alternation for Matcher {
    
    fn has_a_match(&self, pattern: &str, line: str) -> bool {
        let values = pattern.split(ALTERNATION);
        for value in values {
            if line.contains(value) {
                return true
            }
        }
        false
    }
}