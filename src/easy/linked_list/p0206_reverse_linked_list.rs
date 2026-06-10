#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next;

        node.next = prev;
        prev = Some(node);
        curr = next;
    }
    prev
}

#[allow(dead_code)]
fn from_list_to_nodes(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }

    head
}

#[cfg(test)]
mod tests {
    use crate::easy::linked_list::p0206_reverse_linked_list::*;

    #[test]
    fn leetcode_reverse_list_test() {
        assert_eq!(
            reverse_list(from_list_to_nodes(&[1, 2, 3, 4, 5])),
            from_list_to_nodes(&[5, 4, 3, 2, 1])
        );
        assert_eq!(
            reverse_list(from_list_to_nodes(&[1, 2])),
            from_list_to_nodes(&[2, 1])
        );
        assert_eq!(
            reverse_list(from_list_to_nodes(&[])),
            from_list_to_nodes(&[])
        );
    }
}
