// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    let list1_node3 = ListNode::new(4);
    let mut list1_node2 = ListNode::new(2);
    list1_node2.next = Some(Box::new(list1_node3));
    let mut list1_node1 = ListNode::new(1);
    list1_node1.next = Some(Box::new(list1_node2));

    let list2_node3 = ListNode::new(4);
    let mut list2_node2 = ListNode::new(3);
    list2_node2.next = Some(Box::new(list2_node3));
    let mut list2_node1 = ListNode::new(1);
    list2_node1.next = Some(Box::new(list2_node2));

    let sorted = merge_two_lists(Some(Box::new(list1_node1)), Some(Box::new(list2_node1)));
    println!("{:?}", sorted);
}

// pub fn merge_two_lists(
//     mut list1: Option<Box<ListNode>>,
//     mut list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     if list1.is_none() && list2.is_none() {
//         return None;
//     }
//     if list1.is_none() {
//         return list2;
//     }
//     if list2.is_none() {
//         return list1;
//     }

//     let mut dummy = Box::new(ListNode::new(0));
//     let mut tail = &mut dummy;

//     while list1.is_some() && list2.is_some() {
//         let take_from_list1 = list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;

//         if take_from_list1 {
//             let next = list1.as_mut().unwrap().next.take();
//             tail.next = list1;
//             tail = tail.next.as_mut().unwrap();
//             list1 = next;
//         } else {
//             let next = list2.as_mut().unwrap().next.take();
//             tail.next = list2;
//             tail = tail.next.as_mut().unwrap();
//             list2 = next;
//         }
//     }

//     tail.next = if list1.is_some() { list1 } else { list2 };

//     dummy.next
// }

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // Use a dummy node to simplify building the merged list
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    // Local mutable variables for iteration
    let mut list1_node = list1;
    let mut list2_node = list2;

    while list1_node.is_some() && list2_node.is_some() {
        if list1_node.as_ref().unwrap().val <= list2_node.as_ref().unwrap().val {
            let mut node = list1_node.take().unwrap();
            let next = node.next.take(); // take the next node BEFORE moving node
            tail.next = Some(node);
            list1_node = next;
        } else {
            let mut node = list2_node.take().unwrap();
            let next = node.next.take();
            tail.next = Some(node);
            list2_node = next;
        }

        tail = tail.next.as_mut().unwrap();
    }

    tail.next = if list1_node.is_some() {
        list1_node
    } else {
        list2_node
    };

    dummy.next
}
