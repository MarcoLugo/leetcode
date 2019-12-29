// Given head which is a reference node to a singly-linked list. The value of each
// node in the linked list is either 0 or 1. The linked list holds the binary
// representation of a number.

// Return the decimal value of the number in the linked list.

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut next = head;
        let mut result = 0;
        let mut counter = 0;
        let base: i32 = 2;
        let mut values = vec![];
        
        while let Some(node) = next {
            values.push(node.val);
            next = node.next;
        }
        
        while let Some(value) = values.pop() {
            result += value * base.pow(counter);
            counter += 1;
        }
        
        result
    }
}