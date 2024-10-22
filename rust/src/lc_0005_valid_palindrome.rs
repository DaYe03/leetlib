pub fn is_palindrome(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.to_lowercase().chars(){
        if c.is_alphanumeric() {
            stack.push(c);
        }
    } 

    for i in 0..stack.len() {
        if stack[i] != stack[stack.len() - 1 -i] {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case 1: Input with punctuation and spaces, which is a palindrome
    #[test]
    fn test_is_palindrome_case_1() {
        let input = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(input), true);  // Output should be true
    }

    // Test case 2: Input which is not a palindrome
    #[test]
    fn test_is_palindrome_case_2() {
        let input = String::from("race a car");
        assert_eq!(is_palindrome(input), false); // Output should be false
    }

    // Test case 3: Input that is an empty string or whitespace
    #[test]
    fn test_is_palindrome_case_3() {
        let input = String::from(" ");
        assert_eq!(is_palindrome(input), true);  // Output should be true
    }
}