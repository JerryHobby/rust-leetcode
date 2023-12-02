
/////////////////////////////////////////////////////////////
// 191. Number of 1 Bits
// Difficulty: Easy

// Write a function that takes an unsigned integer and returns the number of '1' bits it has
// (also known as the Hamming weight).

use std::time;

struct Solution {}

static MODULE_NAME: &str = "number_of_1_bits_191";

#[allow(dead_code)]

fn main() {
    println!("\n\n<*>-<*>-<*> {}::run() <*>-<*>-<*>", MODULE_NAME);
    let now = time::Instant::now();
    run();
    let elapsed_time = now.elapsed();
    println!("<*>-<*>-<*> Finished in {} microseconds. <*>-<*>-<*>", elapsed_time.as_micros());
}

pub fn run() {
    struct TestCase {
        test: u32,
        result: i32,
    }

    let test_cases = [
        TestCase {
            test: 0b00000000000000000000000000001011,
            result: 3,
        },
        TestCase {
            test: 0b00000000000000000000000010000000,
            result: 1,
        },
        TestCase {
            test: 0b11111111111111111111111111111101,
            result: 31,
        },
    ];

    println!("{}::run() <*>-<*>-<*>-<*>-<*>-<*>-<*>", MODULE_NAME);

    for i in 0..test_cases.len() {
        if Solution::hammingWeight(test_cases[i].test) == test_cases[i].result {
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
    pub fn hammingWeight (mut n: u32) -> i32 {
        let mut count = 0;
        while n != 0 {
            count += 1;
            n &= n - 1;
        }
        count
    }
}
