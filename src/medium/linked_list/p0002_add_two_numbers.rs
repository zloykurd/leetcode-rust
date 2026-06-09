#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let values_l1 = to_vec(l1);
    let values_l2 = to_vec(l2);

    let mut values = vec![];
    let mut carry = 0;

    let n = values_l1.len().max(values_l2.len());
    for i in 0..n {
        let a = values_l1.get(i).copied().unwrap_or(0);
        let b = values_l2.get(i).copied().unwrap_or(0);
        let sum = a + b + carry;
        values.push(sum % 10);
        carry = sum / 10;
    }

    if carry > 0 {
        values.push(carry);
    }

    from_list_to_nodes(&values)
}

fn from_list_to_nodes(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }

    head
}

fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut current = list;
    while let Some(node) = current {
        values.push(node.val);
        current = node.next;
    }
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leetcode_add_two_numbers_tests() {
        assert_eq!(
            add_two_numbers(
                from_list_to_nodes(&[2, 4, 3]),
                from_list_to_nodes(&[5, 6, 4])
            ),
            from_list_to_nodes(&[7, 0, 8])
        );
        assert_eq!(
            add_two_numbers(from_list_to_nodes(&[0]), from_list_to_nodes(&[0])),
            from_list_to_nodes(&[0])
        );
        assert_eq!(
            add_two_numbers(
                from_list_to_nodes(&[9, 9, 9, 9, 9, 9, 9]),
                from_list_to_nodes(&[9, 9, 9, 9])
            ),
            from_list_to_nodes(&[8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
