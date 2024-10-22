pub fn is_valid(s: String) -> bool {
    let mut is_valid = true;
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '{' || c == '[' || c == '(' {
            stack.push(c);
        } else {
            match c {
                ']' => is_valid = stack.pop() == Option::Some('['),
                '}' => is_valid = stack.pop() == Option::Some('{'),
                ')' => is_valid = stack.pop() == Option::Some('('),
                _ => (),
            }
            if !is_valid {
                return false;
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_incorrect() {
        let expected = false;
        let actual = is_valid("(]".to_string());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_valid_correct() {
        let expected = true;
        let actual = is_valid("([])".to_string());
        assert_eq!(expected, actual);
    }
}
