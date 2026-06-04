use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        let complement = target - num;
        match map.get(&complement) {
            Some(&value) => {
                return vec![value as i32, index as i32];
            }

            None => {
                map.insert(*num, index);
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use crate::easy::p0001_two_sum::two_sum;

    #[test]
    fn leetcode_two_sum_test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn leetcode_two_sum_test2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }
    #[test]
    fn leetcode_two_sum_test3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn leetcode_two_sum_test4() {
        let nums = vec![3, 2, 4];
        let target = 7;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 2]);
    }
}
