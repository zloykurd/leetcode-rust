use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut storage: HashMap<char, usize> = HashMap::new();
    let mut length = 0;
    let mut left = 0;
    let mut right = 0;

    for i in s.chars() {
        match storage.contains_key(&i) {
            true => {
                let len = s[left..right].len();
                if len > length {
                    length = len;
                }

                let res = *storage.get(&i).unwrap() + 1;
                if res > left {
                    left = res;
                }
                storage.insert(i, right);
            }
            false => {
                storage.insert(i, right);
            }
        }
        right += 1;
    }
    let len = s[left..right].len();
    if len > length {
        length = len;
    }

    length as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_length_1() {
        let s = String::from("bbbbb");
        let result = length_of_longest_substring(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn should_return_length_2() {
        let s = String::from("abba");
        let result = length_of_longest_substring(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn should_return_length_3() {
        let s = String::from("abcabcbb");
        let result = length_of_longest_substring(s);
        assert_eq!(result, 3);
        let s = String::from("pwwkew");
        let result = length_of_longest_substring(s);
        assert_eq!(result, 3);
    }

    #[test]
    fn should_return_length_5() {
        let s = String::from("xyzxab");
        let result = length_of_longest_substring(s);
        assert_eq!(result, 5);
    }
}
