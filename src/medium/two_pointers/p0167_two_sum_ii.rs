pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        if (numbers[left] + numbers[right]) > target {
            right -= 1;
        } else if (numbers[left] + numbers[right]) < target {
            left += 1;
        } else {
            return vec![left as i32 + 1, right as i32 + 1];
        }
    }

    vec![left as i32 + 1, right as i32 + 1]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn leetcode_two_sum_tests() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(numbers, target);
        assert_eq!(vec![1, 2], result);
        // Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].

        let numbers = vec![2, 3, 4];
        let target = 6;
        let result = two_sum(numbers, target);
        assert_eq!(vec![1, 3], result);
        // Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].

        let numbers = vec![-1, 0];
        let target = -1;
        let result = two_sum(numbers, target);
        assert_eq!(vec![1, 2], result);
        // Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].

        // Constraints:
        //
        // 2 <= numbers.length <= 3 * 104
        // -1000 <= numbers[i] <= 1000
        // numbers is sorted in non-decreasing order.
        // -1000 <= target <= 1000
        // The tests are generated such that there is exactly one solution.
    }
}
