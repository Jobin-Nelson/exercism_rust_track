pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '{'|'['|'(' => stack.push(c),
            '}' => if stack.pop() != Some('{') { return false },
            ']' => if stack.pop() != Some('[') { return false },
            ')' => if stack.pop() != Some('(') { return false },
            _ => (),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }
    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }
    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }
    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }
    #[test]
    fn wrong_closing_brackets() {
        assert!(!brackets_are_balanced("{]"));
    }
    #[test]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }
    #[test]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }
    #[test]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }
    #[test]
    fn paired_with_nested_whitespace() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }
}
