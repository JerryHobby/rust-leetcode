/////////////////////////////////////////////////////////////
// 876. Middle of the Linked List
// https://leetcode.com/problems/middle-of-the-linked-list/description/

use std::time;

static MODULE_NAME: &str = "middle_of_the_linked_list_876";

#[allow(dead_code)]
#[allow(non_snake_case)]

fn main() {
    println!("\n\n<*>-<*>-<*> {}::run() <*>-<*>-<*>", MODULE_NAME);
    let now = time::Instant::now();
    run();
    let elapsed_time = now.elapsed();
    println!("<*>-<*>-<*> Finished in {} microseconds. <*>-<*>-<*>", elapsed_time.as_micros());
}

struct Solution {}

pub fn run() {
    struct TestCase {
        a: Option<Box<ListNode>>,
        result: i32,
    }

    let mut test_cases = [
        TestCase {
            a: Some(Box::new(ListNode { val: 1,
                    next: Some(Box::new(ListNode { val: 2,
                        next: Some(Box::new(ListNode { val: 3,
                            next: Some(Box::new(ListNode { val: 4,
                                next: Some(Box::new(ListNode { val: 5,
                                    next: None
                                }))
                            }))
                        }))
                    }))
                })),
            result: 3
        },

        TestCase {
            a: Some(Box::new(ListNode { val: 1,
                next: Some(Box::new(ListNode { val: 2,
                    next: Some(Box::new(ListNode { val: 3,
                        next: Some(Box::new(ListNode { val: 4,
                            next: Some(Box::new(ListNode { val: 5,
                                next: Some(Box::new(ListNode { val: 6,
                                    next: None
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            result: 4
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        let x = Solution::middle_node(test_cases[i].a.take());
        //println!("x = {}", x.unwrap().val);

        if x.unwrap().val == test_cases[i].result {
            println!("Test case {} passed", i + 1);
        } else {
            println!("Test case {} FAILED", i + 1);
        }
    }
}


/////////////////////////////////////////////////////////////
// Solution implementation

#[allow(non_snake_case)]

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let list_len = (list_length(&head) / 2) as i32;
        newlist(head, list_len)
    }
}

fn newlist(mut head: Option<Box<ListNode>>, len: i32) -> Option<Box<ListNode>> {
    let mut new: Option<Box<ListNode>> = None;
    let mut count = 0;
    let mut curr = &mut new;
    while let Some(mut n) = head.take() {
        if count >= len {
            let node = Box::new(ListNode::new(n.val));
            *curr = Some(node);
            curr = &mut curr.as_mut().unwrap().next;
        }

        head = n.next.take();
        count += 1;
    }
    new
}
fn list_length(mut head: &Option<Box<ListNode>>) -> usize {
    let mut length = 0;
    while let Some(node) = head {
        length += 1;
        head = &node.next;
    }
    length
}