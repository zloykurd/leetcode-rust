#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, value) => value,
        (value, None) => value,
        (Some(mut l1), Some(mut l2)) => {
            if l1.val <= l2.val {
                l1.next = merge_two_lists(l1.next, Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists(Some(l1), l2.next);
                Some(l2)
            }
        }
    }
}

#[allow(unused)]
fn from_list_to_nodes(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }

    head
}

#[allow(unused)]
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
    fn leetcode_merge_two_lists_test() {
        assert_eq!(
            merge_two_lists(
                from_list_to_nodes(&[1, 2, 4]),
                from_list_to_nodes(&[1, 3, 4])
            ),
            from_list_to_nodes(&[1, 1, 2, 3, 4, 4])
        );

        assert_eq!(
            merge_two_lists(from_list_to_nodes(&[2]), from_list_to_nodes(&[1])),
            from_list_to_nodes(&[1, 2])
        );

        assert_eq!(
            merge_two_lists(from_list_to_nodes(&[]), from_list_to_nodes(&[]),),
            from_list_to_nodes(&[])
        );

        assert_eq!(
            merge_two_lists(from_list_to_nodes(&[]), from_list_to_nodes(&[0]),),
            from_list_to_nodes(&[0])
        );
    }
}
