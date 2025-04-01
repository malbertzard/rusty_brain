pub fn tokenize(code: &str) -> Vec<char> {
    code.chars().filter(|c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']')).collect()
}
