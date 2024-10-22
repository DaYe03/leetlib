#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    return merge_rec(list1, list2);
}

fn merge_rec(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (Some(x), Some(y)) => {
            let mut node;
            if x.val < y.val {
                node = Box::new(ListNode::new(x.val));
                node.next = merge_rec(x.next, Some(y));
            } else {
                node = Box::new(ListNode::new(y.val));
                node.next = merge_rec(Some(x), y.next);
            }
            Some(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case 1: list1 = [1,2,4], list2 = [1,3,4]
    #[test]
    fn test_merge_two_lists_case_1() {
        let list1 = vec_to_list(vec![1, 2, 4]);
        let list2 = vec_to_list(vec![1, 3, 4]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    // Test case 2: list1 = [], list2 = []
    #[test]
    fn test_merge_two_lists_case_2() {
        let list1 = vec_to_list(vec![]);
        let list2 = vec_to_list(vec![]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![]);
    }

    // Test case 3: list1 = [], list2 = [0]
    #[test]
    fn test_merge_two_lists_case_3() {
        let list1 = vec_to_list(vec![]);
        let list2 = vec_to_list(vec![0]);
        let merged = merge_two_lists(list1, list2);
        assert_eq!(list_to_vec(merged), vec![0]);
    }

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = current;
            current = Some(new_node);
        }
        current
    }

    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }
}
