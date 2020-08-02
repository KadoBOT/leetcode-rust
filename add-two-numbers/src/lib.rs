#[cfg(test)]
mod tests {
    use crate::{add_two_numbers, ListNode};

    #[test]
    fn it_works() {
        let mut n1 = ListNode::new(2);
        let mut n2 = ListNode::new(4);
        let n3 = ListNode::new(3);
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        let l1 = Some(Box::new(n1));

        let mut n1 = ListNode::new(5);
        let mut n2 = ListNode::new(6);
        let n3 = ListNode::new(4);
        n2.next = Some(Box::new(n3));
        n1.next = Some(Box::new(n2));

        let l2 = Some(Box::new(n1));

        let n1 = &add_two_numbers(l1, l2).unwrap();
        let n2 = n1.next.as_ref().unwrap();
        let n3 = n2.next.as_ref().unwrap();

        assert_eq!(7, n1.val);
        assert_eq!(0, n2.val);
        assert_eq!(8, n3.val);
        assert_eq!(None, n3.next);
    }
}

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        return None;
    }

    pub fn get_result(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        acc: i32,
    ) -> Option<Box<ListNode>> {
        let l1 = l1.unwrap_or_else(|| Box::new(ListNode::new(0)));
        let l2 = l2.unwrap_or_else(|| Box::new(ListNode::new(0)));

        let total = l1.val + l2.val + acc;
        let mut next = None;
        if l1.next.is_some() || l2.next.is_some() {
            let new_acc = match total >= 10 {
                true => 1,
                false => 0,
            };
            next = get_result(l1.next, l2.next, new_acc);
        } else if total >= 10 {
            next = Some(Box::new(ListNode::new(1)));
        }

        let mut result = ListNode::new(total % 10);
        result.next = next;
        Some(Box::new(result))
    }

    get_result(l1, l2, 0)
}
