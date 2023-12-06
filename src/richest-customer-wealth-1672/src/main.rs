/////////////////////////////////////////////////////////////
// 1611. Minimum One Bit Operations to Make Integers Zero
// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/

use std::time;

static MODULE_NAME: &str = "richest-customer-wealth-1672";

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
        a: Vec<Vec<i32>>,
        result: i32,
    }

    let test_cases = [
        TestCase {
            a: [[1,2,3].to_vec(),[3,2,1].to_vec()].to_vec(),
            result: 6,
        },
        TestCase {
            a: [[1,5].to_vec(),[7,3].to_vec(),[3,5].to_vec()].to_vec(),
            result: 10,
        },
        TestCase {
            a: [[2,8,7].to_vec(),[7,1,3].to_vec(),[1,9,5].to_vec()].to_vec(),
            result: 17,
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::maximum_wealth(
            test_cases[i].a.clone()) == test_cases[i].result {
            println!("Test case {} passed", i + 1);
        } else {
            println!("Test case {} FAILED", i + 1);
        }
    }
}


/////////////////////////////////////////////////////////////
// Solution implementation

#[allow(non_snake_case)]

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
     accounts.iter().map(|x| x.iter().sum::<i32>()).max().unwrap()
    }
}
