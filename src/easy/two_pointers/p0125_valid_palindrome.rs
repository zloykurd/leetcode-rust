pub fn is_palindrome(s: String) -> bool {
    let input: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic() || x.is_ascii_lowercase() || x.is_ascii_digit())
        .collect();
    if input.is_empty() {
        return true;
    }

    let (mut left, mut right) = (0, input.len() - 1);

    while left < right {
        if input[left] != input[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::easy::two_pointers::p0125_valid_palindrome::is_palindrome;

    #[test]
    fn test_is_palindrome_returns_true() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(is_palindrome(s));
        let s = String::from(" ");
        assert!(is_palindrome(s));
    }

    #[test]
    fn test_is_palindrome_returns_false() {
        let s = String::from("race a car");
        assert!(!is_palindrome(s));
    }
}
