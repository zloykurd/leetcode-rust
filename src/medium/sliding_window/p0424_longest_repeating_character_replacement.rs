pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut counts: [i32; 26] = [0; 26];
    let mut left = 0;
    let mut result = 0;

    for (item_index, item) in s.char_indices() {
        let letter_index = (item as u8 - b'A') as usize;
        counts[letter_index] += 1;

        let max_count = *counts.iter().max().unwrap();
        let length = s[left..=item_index].len() as i32;
        if (length - max_count) <= k {
            result = result.max(length);
        } else {
            let leaving_letter = s.as_bytes()[left];
            let leaving_index = (leaving_letter - b'A') as usize;
            counts[leaving_index] -= 1;
            left += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_4_with_k_2() {
        let s: String = "ABAB".into();
        let k: i32 = 2;
        let result = character_replacement(s, k);
        assert_eq!(4, result);
    }

    #[test]
    fn should_return_4_with_k_1() {
        let s: String = "AABABBA".into();
        let k: i32 = 1;
        let result = character_replacement(s, k);
        assert_eq!(4, result);
    }
}
