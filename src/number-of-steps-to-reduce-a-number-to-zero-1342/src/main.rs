/////////////////////////////////////////////////////////////
// 1342. Number of Steps to Reduce a Number to Zero
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/description/

use std::time;

static MODULE_NAME: &str = "number-of-steps-to-reduce-a-number-to-zero-1342";

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
        a: i32,
        result: i32,
    }

    let test_cases = [
        TestCase {
            a: 14,
            result: 6,
        },
        TestCase {
            a: 8,
            result: 4,
        },
        TestCase {
            a: 123,
            result: 12,
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::number_of_steps(test_cases[i].a) == test_cases[i].result {
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
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut result:i32 = 0;

        while n > 0 {
            if n % 2 == 0 {
                n/= 2;
            } else {
                n -= 1;
            }
            result += 1;
        }
        result
    }
}