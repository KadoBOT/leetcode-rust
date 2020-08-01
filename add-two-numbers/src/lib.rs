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

fn reverse(n: i64) -> i64 {
    let mut num = n;
    let mut reversed = 0;

    while num != 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    reversed
}

fn get_result(list: Option<Box<ListNode>>) -> i64 {
    let mut result: Vec<String> = vec![];
    let mut list = list;
    while let Some(l) = list {
        result.push(reverse(l.val.into()).to_string());
        list = l.next;
    }

    result.reverse();
    let num_str = result.join("");
    num_str.parse().unwrap()
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list: Option<Box<ListNode>> = None;
    let result_one = get_result(l1);
    let result_two = get_result(l2);
    let total_vec = (result_one + result_two)
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    for num in total_vec.iter() {
        let mut new_list = ListNode::new(*num as i32);
        new_list.next = list;
        list = Some(Box::new(new_list));
    }
    list
}
