use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    for &n in nums.iter() {
        if !seen.insert(n) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::easy::arrays_hashing::p0217_contains_duplicate::contains_duplicate;

    #[test]
    fn leetcode_contains_duplicate_return_true() {
        let nums = vec![1, 2, 3, 1];
        assert!(contains_duplicate(nums));
    }

    #[test]
    fn leetcode_contains_duplicate_return_false() {
        let nums = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(nums));
        let nums = vec![1];
        assert!(!contains_duplicate(nums));
        let nums = vec![];
        assert!(!contains_duplicate(nums));
    }
}
