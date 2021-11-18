//brackets balanced
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut parenthesis: Vec<char> = Vec::new();
    for ch in string.chars() {
        match ch {
            '(' | '{' | '[' => parenthesis.push(ch),
            ')' => {
                if parenthesis.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if parenthesis.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if parenthesis.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    parenthesis.is_empty()
}
