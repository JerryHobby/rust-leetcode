/////////////////////////////////////////////////////////////
// 1480. Running Sum of 1d Array
// https://leetcode.com/problems/running-sum-of-1d-array/description/

use std::time;

static MODULE_NAME: &str = "running-sum-of-1d-array-1480";

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
        a: Vec<i32>,
        result: Vec<i32>,
    }

    let test_cases = [
        TestCase {
            a: [1,2,3,4].to_vec(),
            result: [1,3,6,10].to_vec(),
        },
        TestCase {
            a: [1,1,1,1,1].to_vec(),
            result: [1,2,3,4,5].to_vec(),
        },
        TestCase {
            a: [3,1,2,10,1].to_vec(),
            result: [3,4,6,16,17].to_vec(),
        },
    ];

    for i in 0..test_cases.len() {
        // rename solution
        if Solution::running_sum(
            test_cases[i].a.clone()) == test_cases[i].result {
            println!("Test case {} passed", i);
        } else {
            println!("Test case {} FAILED", i);
        }
    }
}


/////////////////////////////////////////////////////////////
// Solution implementation

#[allow(non_snake_case)]

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {

        let mut result: Vec<i32> = Vec::new();
        let mut sum: i32 = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            result.push(sum);
        }
        result
    }
}