///Brackets balanced for more information [here](https://en.wikipedia.org/wiki/Bracket_matching) .
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

///Remove whitespace from String.
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

