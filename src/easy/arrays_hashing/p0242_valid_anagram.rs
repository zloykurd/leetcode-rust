use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();

    for (a, b) in s.chars().zip(t.chars()) {
        *map.entry(a).or_insert(0) += 1;
        *map.entry(b).or_insert(0) -= 1;
    }

    map.values().all(|x| *x == 0)
}

#[cfg(test)]
mod tests {
    use crate::easy::arrays_hashing::p0242_valid_anagram::is_anagram;

    #[test]
    fn leetcode_is_anagram_return_true() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn leetcode_is_anagram_return_false() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
        assert!(!is_anagram("aab".to_string(), "abb".to_string()));
        assert!(!is_anagram("baba".to_string(), "abb".to_string()));
    }
}
